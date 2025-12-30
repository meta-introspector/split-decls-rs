// Generated macro for PrivateKeyUsagePeriod (struct)
macro_rules! Depcrate_extensionsPrivateKeyUsagePeriod {
() => {
// Module: crate::extensions
// Provides: {"PrivateKeyUsagePeriod"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct PrivateKeyUsagePeriod { # [implicit (0)] pub not_before : Option < asn1 :: X509GeneralizedTime > , # [implicit (1)] pub not_after : Option < asn1 :: X509GeneralizedTime > , }
};
}
