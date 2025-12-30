// Generated macro for impl_124 (impl)
macro_rules! Depcrate_threadimpl_124 {
() => {
// Module: crate::thread
// Provides: {"impl_124"}
// Dependencies: {}
impl allocatedp_mib { # [doc = " Reads value using MIB API."] pub fn read (& self) -> Result < ThreadLocal < u64 > > { unsafe { read_mib (self . 0 . as_ref ()) . map (ThreadLocal) } } }
};
}
