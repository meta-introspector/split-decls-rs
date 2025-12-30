// Generated macro for DistributionPoint (struct)
macro_rules! Depcrate_extensionsDistributionPoint {
() => {
// Module: crate::extensions
// Provides: {"DistributionPoint"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct DistributionPoint < 'a , Op : Asn1Operation > { # [explicit (0)] pub distribution_point : Option < DistributionPointName < 'a , Op > > , # [implicit (1)] pub reasons : crl :: ReasonFlags < 'a , Op > , # [implicit (2)] pub crl_issuer : Option < name :: SequenceOfGeneralName < 'a , Op > > , }
};
}
