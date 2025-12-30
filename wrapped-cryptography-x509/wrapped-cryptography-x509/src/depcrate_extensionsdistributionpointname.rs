// Generated macro for DistributionPointName (enum)
macro_rules! Depcrate_extensionsDistributionPointName {
() => {
// Module: crate::extensions
// Provides: {"DistributionPointName"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub enum DistributionPointName < 'a , Op : Asn1Operation > { # [implicit (0)] FullName (name :: SequenceOfGeneralName < 'a , Op >) , # [implicit (1)] NameRelativeToCRLIssuer (Op :: SetOfVec < 'a , common :: AttributeTypeValue < 'a > >) , }
};
}
