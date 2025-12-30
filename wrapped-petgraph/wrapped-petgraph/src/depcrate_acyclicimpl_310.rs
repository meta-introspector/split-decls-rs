// Generated macro for impl_310 (impl)
macro_rules! Depcrate_acyclicimpl_310 {
() => {
// Module: crate::acyclic
// Provides: {"impl_310"}
// Dependencies: {}
impl < G : Visitable + EdgeCount > EdgeCount for Acyclic < G > { fn edge_count (& self) -> usize { self . inner () . edge_count () } }
};
}
