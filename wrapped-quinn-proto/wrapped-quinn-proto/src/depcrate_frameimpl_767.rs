// Generated macro for impl_767 (impl)
macro_rules! Depcrate_frameimpl_767 {
() => {
// Module: crate::frame
// Provides: {"impl_767"}
// Dependencies: {}
impl NewConnectionId { pub (crate) fn encode < W : BufMut > (& self , out : & mut W) { out . write (FrameType :: NEW_CONNECTION_ID) ; out . write_var (self . sequence) ; out . write_var (self . retire_prior_to) ; out . write (self . id . len () as u8) ; out . put_slice (& self . id) ; out . put_slice (& self . reset_token) ; } }
};
}
