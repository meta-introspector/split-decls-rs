// Generated macro for impl_705 (impl)
macro_rules! Depcrate_frameimpl_705 {
() => {
// Module: crate::frame
// Provides: {"impl_705"}
// Dependencies: {}
impl coding :: Codec for FrameType { fn decode < B : Buf > (buf : & mut B) -> coding :: Result < Self > { Ok (Self (buf . get_var () ?)) } fn encode < B : BufMut > (& self , buf : & mut B) { buf . write_var (self . 0) ; } }
};
}
