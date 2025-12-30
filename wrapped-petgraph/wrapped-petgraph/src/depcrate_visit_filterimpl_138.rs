// Generated macro for impl_138 (impl)
macro_rules! Depcrate_visit_filterimpl_138 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_138"}
// Dependencies: {}
impl < F , N > FilterNode < N > for F where F : Fn (N) -> bool , { fn include_node (& self , n : N) -> bool { (* self) (n) } }
};
}
