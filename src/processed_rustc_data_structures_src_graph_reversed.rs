// SRC: ../rust/compiler/rustc_data_structures/src/graph/reversed.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::graph::{DirectedGraph, Predecessors, Successors};
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=ReversedGraph | COMPLEXITY=6 | LINES=10 */

/// View that reverses the direction of edges in its underlying graph, so that
/// successors become predecessors and vice-versa.
///
/// Because of `impl<G: Graph> Graph for &G`, the underlying graph can be
/// wrapped by-reference instead of by-value if desired.
#[derive(Clone, Copy, Debug)]
pub struct ReversedGraph<G> {
    pub inner: G,
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=6 */

impl<G> ReversedGraph<G> {
    pub fn new(inner: G) -> Self {
        Self { inner }
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=num_nodes | COMPLEXITY=5 | LINES=8 */

impl<G: DirectedGraph> DirectedGraph for ReversedGraph<G> {
    type Node = G::Node;

    fn num_nodes(&self) -> usize {
        self.inner.num_nodes()
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=successors | COMPLEXITY=7 | LINES=11 */

// Implementing `StartNode` is not possible in general, because the start node
// of an underlying graph is instead an _end_ node in the reversed graph.
// But would be possible to define another wrapper type that adds an explicit
// start node to its underlying graph, if desired.

impl<G: Predecessors> Successors for ReversedGraph<G> {
    fn successors(&self, node: Self::Node) -> impl Iterator<Item = Self::Node> {
        self.inner.predecessors(node)
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=predecessors | COMPLEXITY=5 | LINES=6 */

impl<G: Successors> Predecessors for ReversedGraph<G> {
    fn predecessors(&self, node: Self::Node) -> impl Iterator<Item = Self::Node> {
        self.inner.successors(node)
    }
}