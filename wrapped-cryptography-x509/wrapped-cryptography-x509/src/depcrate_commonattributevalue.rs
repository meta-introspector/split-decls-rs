// Generated macro for AttributeValue (enum)
macro_rules! Depcrate_commonAttributeValue {
() => {
// Module: crate::common
// Provides: {"AttributeValue"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , PartialEq , Eq , Hash , Clone)] pub enum AttributeValue < 'a > { UniversalString (asn1 :: UniversalString < 'a >) , BmpString (asn1 :: BMPString < 'a >) , PrintableString (asn1 :: PrintableString < 'a >) , AnyString (RawTlv < 'a >) , }
};
}
