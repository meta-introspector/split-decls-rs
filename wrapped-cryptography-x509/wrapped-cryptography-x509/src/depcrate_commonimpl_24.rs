// Generated macro for impl_24 (impl)
macro_rules! Depcrate_commonimpl_24 {
() => {
// Module: crate::common
// Provides: {"impl_24"}
// Dependencies: {}
impl asn1 :: Asn1Writable for RawTlv < '_ > { fn write (& self , w : & mut asn1 :: Writer < '_ >) -> asn1 :: WriteResult { w . write_tlv (self . tag , Some (self . value . len ()) , move | dest | { dest . push_slice (self . value) }) } fn encoded_length (& self) -> Option < usize > { None } }
};
}
