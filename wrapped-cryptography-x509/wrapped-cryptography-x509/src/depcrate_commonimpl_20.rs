// Generated macro for impl_20 (impl)
macro_rules! Depcrate_commonimpl_20 {
() => {
// Module: crate::common
// Provides: {"impl_20"}
// Dependencies: {}
impl AttributeValue < '_ > { pub fn tag (& self) -> asn1 :: Tag { match self { AttributeValue :: AnyString (tlv) => tlv . tag () , AttributeValue :: PrintableString (_) => asn1 :: PrintableString :: TAG , AttributeValue :: UniversalString (_) => asn1 :: UniversalString :: TAG , AttributeValue :: BmpString (_) => asn1 :: BMPString :: TAG , } } }
};
}
