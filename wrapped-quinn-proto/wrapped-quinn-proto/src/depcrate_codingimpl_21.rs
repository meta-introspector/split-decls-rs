// Generated macro for impl_21 (impl)
macro_rules! Depcrate_codingimpl_21 {
() => {
// Module: crate::coding
// Provides: {"impl_21"}
// Dependencies: {}
impl Codec for u64 { fn decode < B : Buf > (buf : & mut B) -> Result < Self > { if buf . remaining () < 8 { return Err (UnexpectedEnd) ; } Ok (buf . get_u64 ()) } fn encode < B : BufMut > (& self , buf : & mut B) { buf . put_u64 (* self) ; } }
};
}
