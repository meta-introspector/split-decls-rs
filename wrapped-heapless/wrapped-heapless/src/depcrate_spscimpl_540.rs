// Generated macro for impl_540 (impl)
macro_rules! Depcrate_spscimpl_540 {
() => {
// Module: crate::spsc
// Provides: {"impl_540"}
// Dependencies: {}
impl < T , const N : usize > Queue < T , N > { # [doc = " Creates an empty queue."] pub const fn new () -> Self { const { assert ! (N > 1) ; } Queue { head : AtomicUsize :: new (0) , tail : AtomicUsize :: new (0) , buffer : [const { UnsafeCell :: new (MaybeUninit :: uninit ()) } ; N] , } } # [doc = " Used in `Storage` implementation"] pub (crate) fn as_view_private (& self) -> & QueueView < T > { self } # [doc = " Used in `Storage` implementation"] pub (crate) fn as_mut_view_private (& mut self) -> & mut QueueView < T > { self } }
};
}
