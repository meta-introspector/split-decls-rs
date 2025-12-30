// Generated macro for impl_885 (impl)
macro_rules! Depcrate_transport_errorimpl_885 {
() => {
// Module: crate::transport_error
// Provides: {"impl_885"}
// Dependencies: {}
impl coding :: Codec for Code { fn decode < B : Buf > (buf : & mut B) -> coding :: Result < Self > { Ok (Self (buf . get_var () ?)) } fn encode < B : BufMut > (& self , buf : & mut B) { buf . write_var (self . 0) } }
};
}
