// Generated macro for impl_22 (impl)
macro_rules! Depcrate_codingimpl_22 {
() => {
// Module: crate::coding
// Provides: {"impl_22"}
// Dependencies: {}
impl Codec for Ipv4Addr { fn decode < B : Buf > (buf : & mut B) -> Result < Self > { if buf . remaining () < 4 { return Err (UnexpectedEnd) ; } let mut octets = [0 ; 4] ; buf . copy_to_slice (& mut octets) ; Ok (octets . into ()) } fn encode < B : BufMut > (& self , buf : & mut B) { buf . put_slice (& self . octets ()) ; } }
};
}
