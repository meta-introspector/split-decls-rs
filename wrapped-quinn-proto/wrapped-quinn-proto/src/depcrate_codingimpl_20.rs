// Generated macro for impl_20 (impl)
macro_rules! Depcrate_codingimpl_20 {
() => {
// Module: crate::coding
// Provides: {"impl_20"}
// Dependencies: {}
impl Codec for u32 { fn decode < B : Buf > (buf : & mut B) -> Result < Self > { if buf . remaining () < 4 { return Err (UnexpectedEnd) ; } Ok (buf . get_u32 ()) } fn encode < B : BufMut > (& self , buf : & mut B) { buf . put_u32 (* self) ; } }
};
}
