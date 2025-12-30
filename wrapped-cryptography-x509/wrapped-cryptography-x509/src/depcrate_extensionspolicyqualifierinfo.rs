// Generated macro for PolicyQualifierInfo (struct)
macro_rules! Depcrate_extensionsPolicyQualifierInfo {
() => {
// Module: crate::extensions
// Provides: {"PolicyQualifierInfo"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct PolicyQualifierInfo < 'a , Op : Asn1Operation > { pub policy_qualifier_id : asn1 :: ObjectIdentifier , pub qualifier : Qualifier < 'a , Op > , }
};
}
