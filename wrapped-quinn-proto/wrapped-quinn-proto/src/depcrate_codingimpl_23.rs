// Generated macro for impl_23 (impl)
macro_rules! Depcrate_codingimpl_23 {
() => {
// Module: crate::coding
// Provides: {"impl_23"}
// Dependencies: {}
impl Codec for Ipv6Addr { fn decode < B : Buf > (buf : & mut B) -> Result < Self > { if buf . remaining () < 16 { return Err (UnexpectedEnd) ; } let mut octets = [0 ; 16] ; buf . copy_to_slice (& mut octets) ; Ok (octets . into ()) } fn encode < B : BufMut > (& self , buf : & mut B) { buf . put_slice (& self . octets ()) ; } }
};
}
