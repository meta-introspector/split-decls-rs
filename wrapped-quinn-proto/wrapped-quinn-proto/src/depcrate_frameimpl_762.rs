// Generated macro for impl_762 (impl)
macro_rules! Depcrate_frameimpl_762 {
() => {
// Module: crate::frame
// Provides: {"impl_762"}
// Dependencies: {}
impl ResetStream { pub (crate) fn encode < W : BufMut > (& self , out : & mut W) { out . write (FrameType :: RESET_STREAM) ; out . write (self . id) ; out . write (self . error_code) ; out . write (self . final_offset) ; } }
};
}
