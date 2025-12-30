// Generated macro for impl_123 (impl)
macro_rules! Depcrate_threadimpl_123 {
() => {
// Module: crate::thread
// Provides: {"impl_123"}
// Dependencies: {}
impl allocatedp { # [doc = " Reads value using string API."] pub fn read () -> Result < ThreadLocal < u64 > > { unsafe { read (Self :: name () . as_bytes ()) . map (ThreadLocal) } } }
};
}
