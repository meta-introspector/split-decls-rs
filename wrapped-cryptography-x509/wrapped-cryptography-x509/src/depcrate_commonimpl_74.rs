// Generated macro for impl_74 (impl)
macro_rules! Depcrate_commonimpl_74 {
() => {
// Module: crate::common
// Provides: {"impl_74"}
// Dependencies: {}
impl < T : asn1 :: Asn1Writable > asn1 :: Asn1Writable for WithTlv < '_ , T > { fn write (& self , w : & mut asn1 :: Writer < '_ >) -> asn1 :: WriteResult < () > { self . value . write (w) } fn encoded_length (& self) -> Option < usize > { self . value . encoded_length () } }
};
}
