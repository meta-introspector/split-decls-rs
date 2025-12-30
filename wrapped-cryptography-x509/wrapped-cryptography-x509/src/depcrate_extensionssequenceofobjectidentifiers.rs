// Generated macro for SequenceOfObjectIdentifiers (type)
macro_rules! Depcrate_extensionsSequenceOfObjectIdentifiers {
() => {
// Module: crate::extensions
// Provides: {"SequenceOfObjectIdentifiers"}
// Dependencies: {}
type SequenceOfObjectIdentifiers < 'a , Op > = < Op as Asn1Operation > :: SequenceOfVec < 'a , asn1 :: ObjectIdentifier > ;
};
}
