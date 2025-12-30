// Generated macro for impl_2685 (impl)
macro_rules! Depcrate_abortableimpl_2685 {
() => {
// Module: crate::abortable
// Provides: {"impl_2685"}
// Dependencies: {}
impl AbortHandle { # [doc = " Creates an (`AbortHandle`, `AbortRegistration`) pair which can be used"] # [doc = " to abort a running future or stream."] # [doc = ""] # [doc = " This function is usually paired with a call to [`Abortable::new`]."] pub fn new_pair () -> (Self , AbortRegistration) { let inner = Arc :: new (AbortInner { waker : AtomicWaker :: new () , aborted : AtomicBool :: new (false) }) ; (Self { inner : inner . clone () } , AbortRegistration { inner }) } }
};
}
