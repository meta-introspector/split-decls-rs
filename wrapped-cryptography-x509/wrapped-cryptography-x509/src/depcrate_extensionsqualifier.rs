// Generated macro for Qualifier (enum)
macro_rules! Depcrate_extensionsQualifier {
() => {
// Module: crate::extensions
// Provides: {"Qualifier"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub enum Qualifier < 'a , Op : Asn1Operation > { CpsUri (asn1 :: IA5String < 'a >) , UserNotice (UserNotice < 'a , Op >) , }
};
}
