// Generated macro for impl_506 (impl)
macro_rules! Depcrate_concurrency_threadimpl_506 {
() => {
// Module: crate::concurrency::thread
// Provides: {"impl_506"}
// Dependencies: {}
impl Idx for ThreadId { fn new (idx : usize) -> Self { ThreadId (u32 :: try_from (idx) . unwrap ()) } fn index (self) -> usize { usize :: try_from (self . 0) . unwrap () } }
};
}
