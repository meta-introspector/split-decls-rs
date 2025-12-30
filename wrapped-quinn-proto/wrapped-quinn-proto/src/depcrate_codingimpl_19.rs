// Generated macro for impl_19 (impl)
macro_rules! Depcrate_codingimpl_19 {
() => {
// Module: crate::coding
// Provides: {"impl_19"}
// Dependencies: {}
impl Codec for u16 { fn decode < B : Buf > (buf : & mut B) -> Result < Self > { if buf . remaining () < 2 { return Err (UnexpectedEnd) ; } Ok (buf . get_u16 ()) } fn encode < B : BufMut > (& self , buf : & mut B) { buf . put_u16 (* self) ; } }
};
}
