// Generated macro for AccessDescription (struct)
macro_rules! Depcrate_extensionsAccessDescription {
() => {
// Module: crate::extensions
// Provides: {"AccessDescription"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct AccessDescription < 'a > { pub access_method : asn1 :: ObjectIdentifier , pub access_location : name :: GeneralName < 'a > , }
};
}
