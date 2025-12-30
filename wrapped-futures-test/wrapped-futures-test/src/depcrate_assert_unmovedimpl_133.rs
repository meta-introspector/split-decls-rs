// Generated macro for impl_133 (impl)
macro_rules! Depcrate_assert_unmovedimpl_133 {
() => {
// Module: crate::assert_unmoved
// Provides: {"impl_133"}
// Dependencies: {}
impl < Fut : FusedFuture > FusedFuture for AssertUnmoved < Fut > { fn is_terminated (& self) -> bool { self . inner . is_terminated () } }
};
}
