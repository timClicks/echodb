// REPO https://github.com/timClicks/echodb/

// use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BTreeMap;

// enum GraphKind {
//     Directed,
//     Undireced,
// }
// kind: GraphKind,

type Node = u64;
type Edge = (Node, Node);

type BackingStore = BTreeMap<Node, BTreeSet<Node>>;

struct DiGraph {
    backing_store: BackingStore,
}

impl DiGraph {
    fn from_edges(edges: Vec<Edge>) -> DiGraph {
        let mut backing_store: BackingStore = BTreeMap::new();

        for edge in edges {
            // TODO: use Entry API
            let source = edge.0;
            let destination = edge.1;
            if !backing_store.contains_key(&source) {
                backing_store.insert(source, BTreeSet::new());
            }
            if !backing_store.contains_key(&destination) {
                backing_store.insert(destination, BTreeSet::new());
            }
            let out_edges = backing_store.get_mut(&source).unwrap();
            out_edges.insert(destination);
        }

        DiGraph {
            backing_store: backing_store,
        }
    }

    fn nodes(&self) -> Vec<Node> {
        let mut all_keys: Vec<u64> = vec![];
        for node in self.backing_store.keys() {
            all_keys.push(*node)
        };
        all_keys
    }

    fn edges(&self) -> Vec<Edge> {
        let mut all_edges: Vec<Edge> = vec![];
        for node_1 in self.nodes() {
            for node_2 in &self.backing_store[&node_1] {
                all_edges.push((node_1, *node_2))
            }
        }
        all_edges
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_startup() {
    let edge_list : Vec<Edge> = vec![(1, 2), (2, 3), (1, 3), (3, 1), (3, 4)];
    let g = DiGraph::from_edges(edge_list.clone());
    let nodes: Vec<_> = g.nodes();
    let edges: Vec<_> = g.edges();

    println!("{:?}", nodes);
    println!("{:?}", edges);

    for edge in &edge_list.clone() {
        assert!(nodes.contains(&edge.0));
        assert!(nodes.contains(&edge.1));
    }
}
