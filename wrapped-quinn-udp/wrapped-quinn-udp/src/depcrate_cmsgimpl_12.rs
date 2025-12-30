// Generated macro for impl_12 (impl)
macro_rules! Depcrate_cmsgimpl_12 {
() => {
// Module: crate::cmsg
// Provides: {"impl_12"}
// Dependencies: {}
impl < M : MsgHdr > Drop for Encoder < '_ , M > { fn drop (& mut self) { self . hdr . set_control_len (self . len as _) ; } }
};
}
