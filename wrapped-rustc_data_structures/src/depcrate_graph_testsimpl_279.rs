// Generated macro for impl_279 (impl)
macro_rules! Depcrate_graph_testsimpl_279 {
() => {
// Module: crate::graph::tests
// Provides: {"impl_279"}
// Dependencies: {}
impl Successors for TestGraph { fn successors (& self , node : usize) -> impl Iterator < Item = Self :: Node > { self . successors [& node] . iter () . cloned () } }
};
}
