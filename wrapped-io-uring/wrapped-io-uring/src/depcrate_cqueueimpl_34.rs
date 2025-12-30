// Generated macro for impl_34 (impl)
macro_rules! Depcrate_cqueueimpl_34 {
() => {
// Module: crate::cqueue
// Provides: {"impl_34"}
// Dependencies: {}
impl < E : EntryMarker > ExactSizeIterator for CompletionQueue < '_ , E > { # [inline] fn len (& self) -> usize { self . tail . wrapping_sub (self . head) as usize } }
};
}
