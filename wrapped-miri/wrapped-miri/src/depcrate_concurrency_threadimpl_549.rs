// Generated macro for impl_549 (impl)
macro_rules! Depcrate_concurrency_threadimpl_549 {
() => {
// Module: crate::concurrency::thread
// Provides: {"impl_549"}
// Dependencies: {}
impl ThreadId { pub fn to_u32 (self) -> u32 { self . 0 } # [doc = " Create a new thread id from a `u32` without checking if this thread exists."] pub fn new_unchecked (id : u32) -> Self { Self (id) } pub const MAIN_THREAD : ThreadId = ThreadId (0) ; }
};
}
