// Generated macro for impl_1752 (impl)
macro_rules! Depcrate_stream_futures_unordered_taskimpl_1752 {
() => {
// Module: crate::stream::futures_unordered::task
// Provides: {"impl_1752"}
// Dependencies: {}
impl < Fut > Task < Fut > { # [doc = " Returns a waker reference for this task without cloning the Arc."] pub (super) unsafe fn waker_ref (this : & Arc < Self >) -> waker_ref :: WakerRef < '_ > { unsafe { waker_ref :: waker_ref (this) } } # [doc = " Spins until `next_all` is no longer set to `pending_next_all`."] # [doc = ""] # [doc = " The temporary `pending_next_all` value is typically overwritten fairly"] # [doc = " quickly after a node is inserted into the list of all futures, so this"] # [doc = " should rarely spin much."] # [doc = ""] # [doc = " When it returns, the correct `next_all` value is returned."] # [doc = ""] # [doc = " `Relaxed` or `Acquire` ordering can be used. `Acquire` ordering must be"] # [doc = " used before `len_all` can be safely read."] # [inline] pub (super) fn spin_next_all (& self , pending_next_all : * mut Self , ordering : Ordering ,) -> * const Self { loop { let next = self . next_all . load (ordering) ; if next != pending_next_all { return next ; } } } }
};
}
