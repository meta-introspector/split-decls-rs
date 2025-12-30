// Generated macro for SequenceOfPolicyQualifiers (type)
macro_rules! Depcrate_extensionsSequenceOfPolicyQualifiers {
() => {
// Module: crate::extensions
// Provides: {"SequenceOfPolicyQualifiers"}
// Dependencies: {}
type SequenceOfPolicyQualifiers < 'a , Op > = < Op as Asn1Operation > :: SequenceOfVec < 'a , PolicyQualifierInfo < 'a , Op > > ;
};
}
