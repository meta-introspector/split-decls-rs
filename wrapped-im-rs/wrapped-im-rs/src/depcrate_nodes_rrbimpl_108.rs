// Generated macro for impl_108 (impl)
macro_rules! Depcrate_nodes_rrbimpl_108 {
() => {
// Module: crate::nodes::rrb
// Provides: {"impl_108"}
// Dependencies: {}
impl Clone for Size { fn clone (& self) -> Self { match * self { Size :: Size (size) => Size :: Size (size) , Size :: Table (ref table) => Size :: Table (table . clone ()) , } } }
};
}
