// Generated macro for Extension (struct)
macro_rules! Depcrate_extensionsExtension {
() => {
// Module: crate::extensions
// Provides: {"Extension"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , PartialEq , Eq , Hash , Clone)] pub struct Extension < 'a > { pub extn_id : asn1 :: ObjectIdentifier , # [default (false)] pub critical : bool , pub extn_value : & 'a [u8] , }
};
}
