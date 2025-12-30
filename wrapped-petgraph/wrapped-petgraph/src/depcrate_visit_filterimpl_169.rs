// Generated macro for impl_169 (impl)
macro_rules! Depcrate_visit_filterimpl_169 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_169"}
// Dependencies: {}
impl < F , N > FilterEdge < N > for F where F : Fn (N) -> bool , { fn include_edge (& self , n : N) -> bool { (* self) (n) } }
};
}
