// Generated macro for impl_22 (impl)
macro_rules! Depcrate_commonimpl_22 {
() => {
// Module: crate::common
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'a > RawTlv < 'a > { pub fn new (tag : asn1 :: Tag , value : & 'a [u8]) -> Self { RawTlv { tag , value } } pub fn tag (& self) -> asn1 :: Tag { self . tag } pub fn data (& self) -> & 'a [u8] { self . value } }
};
}
