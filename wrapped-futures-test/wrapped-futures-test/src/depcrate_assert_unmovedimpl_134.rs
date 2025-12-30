// Generated macro for impl_134 (impl)
macro_rules! Depcrate_assert_unmovedimpl_134 {
() => {
// Module: crate::assert_unmoved
// Provides: {"impl_134"}
// Dependencies: {}
impl < St : Stream > Stream for AssertUnmoved < St > { type Item = St :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . poll_with (| s | s . poll_next (cx)) } }
};
}
