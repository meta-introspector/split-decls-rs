// Generated macro for impl_765 (impl)
macro_rules! Depcrate_frameimpl_765 {
() => {
// Module: crate::frame
// Provides: {"impl_765"}
// Dependencies: {}
impl StopSending { pub (crate) fn encode < W : BufMut > (& self , out : & mut W) { out . write (FrameType :: STOP_SENDING) ; out . write (self . id) ; out . write (self . error_code) } }
};
}
