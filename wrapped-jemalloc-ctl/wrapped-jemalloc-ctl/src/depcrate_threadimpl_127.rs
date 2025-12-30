// Generated macro for impl_127 (impl)
macro_rules! Depcrate_threadimpl_127 {
() => {
// Module: crate::thread
// Provides: {"impl_127"}
// Dependencies: {}
impl deallocatedp_mib { # [doc = " Reads value using MIB API."] pub fn read (& self) -> Result < ThreadLocal < u64 > > { unsafe { read_mib (self . 0 . as_ref ()) . map (ThreadLocal) } } }
};
}
