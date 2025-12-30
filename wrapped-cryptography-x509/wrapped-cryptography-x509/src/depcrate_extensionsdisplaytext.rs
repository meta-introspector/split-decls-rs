// Generated macro for DisplayText (enum)
macro_rules! Depcrate_extensionsDisplayText {
() => {
// Module: crate::extensions
// Provides: {"DisplayText"}
// Dependencies: {}
# [allow (clippy :: enum_variant_names)] # [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub enum DisplayText < 'a > { IA5String (asn1 :: IA5String < 'a >) , Utf8String (asn1 :: Utf8String < 'a >) , VisibleString (common :: UnvalidatedVisibleString < 'a >) , BmpString (asn1 :: BMPString < 'a >) , }
};
}
