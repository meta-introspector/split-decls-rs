// Generated macro for IssuingDistributionPoint (struct)
macro_rules! Depcrate_crlIssuingDistributionPoint {
() => {
// Module: crate::crl
// Provides: {"IssuingDistributionPoint"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct IssuingDistributionPoint < 'a , Op : Asn1Operation > { # [explicit (0)] pub distribution_point : Option < extensions :: DistributionPointName < 'a , Op > > , # [implicit (1)] # [default (false)] pub only_contains_user_certs : bool , # [implicit (2)] # [default (false)] pub only_contains_ca_certs : bool , # [implicit (3)] pub only_some_reasons : ReasonFlags < 'a , Op > , # [implicit (4)] # [default (false)] pub indirect_crl : bool , # [implicit (5)] # [default (false)] pub only_contains_attribute_certs : bool , }
};
}
