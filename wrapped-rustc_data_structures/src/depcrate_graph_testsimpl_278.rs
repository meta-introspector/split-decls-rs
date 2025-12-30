// Generated macro for impl_278 (impl)
macro_rules! Depcrate_graph_testsimpl_278 {
() => {
// Module: crate::graph::tests
// Provides: {"impl_278"}
// Dependencies: {}
impl Predecessors for TestGraph { fn predecessors (& self , node : usize) -> impl Iterator < Item = Self :: Node > { self . predecessors [& node] . iter () . cloned () } }
};
}
