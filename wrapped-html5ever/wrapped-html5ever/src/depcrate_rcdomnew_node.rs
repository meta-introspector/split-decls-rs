// Generated macro for new_node (function)
macro_rules! Depcrate_rcdomnew_node {
() => {
// Module: crate::rcdom
// Provides: {"new_node"}
// Dependencies: {}
fn new_node (node : NodeEnum) -> Handle { Handle (Rc :: new (RefCell :: new (Node :: new (node)))) }
};
}
