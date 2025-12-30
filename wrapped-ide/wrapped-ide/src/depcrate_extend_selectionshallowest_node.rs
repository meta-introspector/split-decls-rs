// Generated macro for shallowest_node (function)
macro_rules! Depcrate_extend_selectionshallowest_node {
() => {
// Module: crate::extend_selection
// Provides: {"shallowest_node"}
// Dependencies: {}
# [doc = " Find the shallowest node with same range, which allows us to traverse siblings."] fn shallowest_node (node : & SyntaxNode) -> SyntaxNode { node . ancestors () . take_while (| n | n . text_range () == node . text_range ()) . last () . unwrap () }
};
}
