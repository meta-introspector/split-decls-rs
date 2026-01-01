// SRC: ../rust/compiler/rustc_data_structures/src/graph/reference.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=num_nodes | COMPLEXITY=5 | LINES=9 */
use super::*;

impl<'graph, G: DirectedGraph> DirectedGraph for &'graph G {
    type Node = G::Node;

    fn num_nodes(&self) -> usize {
        (**self).num_nodes()
    }
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=start_node | COMPLEXITY=5 | LINES=6 */

impl<'graph, G: StartNode> StartNode for &'graph G {
    fn start_node(&self) -> Self::Node {
        (**self).start_node()
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=successors | COMPLEXITY=5 | LINES=6 */

impl<'graph, G: Successors> Successors for &'graph G {
    fn successors(&self, node: Self::Node) -> impl Iterator<Item = Self::Node> {
        (**self).successors(node)
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=predecessors | COMPLEXITY=5 | LINES=6 */

impl<'graph, G: Predecessors> Predecessors for &'graph G {
    fn predecessors(&self, node: Self::Node) -> impl Iterator<Item = Self::Node> {
        (**self).predecessors(node)
    }
}