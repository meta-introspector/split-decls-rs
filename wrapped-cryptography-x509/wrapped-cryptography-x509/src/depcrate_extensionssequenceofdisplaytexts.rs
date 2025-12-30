// Generated macro for SequenceOfDisplayTexts (type)
macro_rules! Depcrate_extensionsSequenceOfDisplayTexts {
() => {
// Module: crate::extensions
// Provides: {"SequenceOfDisplayTexts"}
// Dependencies: {}
type SequenceOfDisplayTexts < 'a , Op > = < Op as Asn1Operation > :: SequenceOfVec < 'a , DisplayText < 'a > > ;
};
}
