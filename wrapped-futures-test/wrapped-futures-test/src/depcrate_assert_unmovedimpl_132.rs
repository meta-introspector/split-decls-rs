// Generated macro for impl_132 (impl)
macro_rules! Depcrate_assert_unmovedimpl_132 {
() => {
// Module: crate::assert_unmoved
// Provides: {"impl_132"}
// Dependencies: {}
impl < Fut : Future > Future for AssertUnmoved < Fut > { type Output = Fut :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . poll_with (| f | f . poll (cx)) } }
};
}
