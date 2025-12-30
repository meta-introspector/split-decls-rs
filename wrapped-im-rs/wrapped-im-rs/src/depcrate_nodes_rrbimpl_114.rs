// Generated macro for impl_114 (impl)
macro_rules! Depcrate_nodes_rrbimpl_114 {
() => {
// Module: crate::nodes::rrb
// Provides: {"impl_114"}
// Dependencies: {}
impl < A : Clone > Clone for Entry < A > { fn clone (& self) -> Self { match * self { Nodes (ref size , ref nodes) => Nodes (size . clone () , nodes . clone ()) , Values (ref values) => Values (values . clone ()) , Empty => Empty , } } }
};
}
