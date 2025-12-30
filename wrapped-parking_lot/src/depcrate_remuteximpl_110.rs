// Generated macro for impl_110 (impl)
macro_rules! Depcrate_remuteximpl_110 {
() => {
// Module: crate::remutex
// Provides: {"impl_110"}
// Dependencies: {}
unsafe impl GetThreadId for RawThreadId { const INIT : RawThreadId = RawThreadId ; fn nonzero_thread_id (& self) -> NonZeroUsize { thread_local ! (static KEY : u8 = 0) ; KEY . with (| x | { NonZeroUsize :: new (x as * const _ as usize) . expect ("thread-local variable address is null") }) } }
};
}
