// SRC: ../rust/compiler/rustc_data_structures/src/graph/tests.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */
use std::cmp::max;

use super::*;
use crate::fx::FxHashMap;

pub(super) struct TestGraph {
    num_nodes: usize,
    start_node: usize,
    successors: FxHashMap<usize, Vec<usize>>,
    predecessors: FxHashMap<usize, Vec<usize>>,
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=11 | LINES=22 */

impl TestGraph {
    pub(super) fn new(start_node: usize, edges: &[(usize, usize)]) -> Self {
        let mut graph = TestGraph {
            num_nodes: start_node + 1,
            start_node,
            successors: FxHashMap::default(),
            predecessors: FxHashMap::default(),
        };
        for &(source, target) in edges {
            graph.num_nodes = max(graph.num_nodes, source + 1);
            graph.num_nodes = max(graph.num_nodes, target + 1);
            graph.successors.entry(source).or_default().push(target);
            graph.predecessors.entry(target).or_default().push(source);
        }
        for node in 0..graph.num_nodes {
            graph.successors.entry(node).or_default();
            graph.predecessors.entry(node).or_default();
        }
        graph
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=num_nodes | COMPLEXITY=5 | LINES=8 */

impl DirectedGraph for TestGraph {
    type Node = usize;

    fn num_nodes(&self) -> usize {
        self.num_nodes
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=start_node | COMPLEXITY=5 | LINES=6 */

impl StartNode for TestGraph {
    fn start_node(&self) -> usize {
        self.start_node
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=predecessors | COMPLEXITY=5 | LINES=6 */

impl Predecessors for TestGraph {
    fn predecessors(&self, node: usize) -> impl Iterator<Item = Self::Node> {
        self.predecessors[&node].iter().cloned()
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=successors | COMPLEXITY=5 | LINES=6 */

impl Successors for TestGraph {
    fn successors(&self, node: usize) -> impl Iterator<Item = Self::Node> {
        self.successors[&node].iter().cloned()
    }
}