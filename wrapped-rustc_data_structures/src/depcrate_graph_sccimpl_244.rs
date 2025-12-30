// Generated macro for impl_244 (impl)
macro_rules! Depcrate_graph_sccimpl_244 {
() => {
// Module: crate::graph::scc
// Provides: {"impl_244"}
// Dependencies: {}
impl < N : Idx , S : Idx + Ord > Successors for Sccs < N , S > { fn successors (& self , node : S) -> impl Iterator < Item = Self :: Node > { self . successors (node) . iter () . cloned () } }
};
}
