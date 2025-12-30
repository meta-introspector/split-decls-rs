// Generated macro for PolicyInformation (struct)
macro_rules! Depcrate_extensionsPolicyInformation {
() => {
// Module: crate::extensions
// Provides: {"PolicyInformation"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct PolicyInformation < 'a , Op : Asn1Operation + 'a > { pub policy_identifier : asn1 :: ObjectIdentifier , pub policy_qualifiers : Option < SequenceOfPolicyQualifiers < 'a , Op > > , }
};
}
