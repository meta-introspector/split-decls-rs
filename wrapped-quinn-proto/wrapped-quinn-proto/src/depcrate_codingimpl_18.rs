// Generated macro for impl_18 (impl)
macro_rules! Depcrate_codingimpl_18 {
() => {
// Module: crate::coding
// Provides: {"impl_18"}
// Dependencies: {}
impl Codec for u8 { fn decode < B : Buf > (buf : & mut B) -> Result < Self > { if buf . remaining () < 1 { return Err (UnexpectedEnd) ; } Ok (buf . get_u8 ()) } fn encode < B : BufMut > (& self , buf : & mut B) { buf . put_u8 (* self) ; } }
};
}
