// Generated macro for impl_315 (impl)
macro_rules! Depcrate_acyclicimpl_315 {
() => {
// Module: crate::acyclic
// Provides: {"impl_315"}
// Dependencies: {}
impl < G : Visitable + NodeCount > NodeCount for Acyclic < G > { fn node_count (& self) -> usize { self . inner () . node_count () } }
};
}
