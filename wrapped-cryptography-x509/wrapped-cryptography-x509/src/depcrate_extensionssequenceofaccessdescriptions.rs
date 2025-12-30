// Generated macro for SequenceOfAccessDescriptions (type)
macro_rules! Depcrate_extensionsSequenceOfAccessDescriptions {
() => {
// Module: crate::extensions
// Provides: {"SequenceOfAccessDescriptions"}
// Dependencies: {}
pub type SequenceOfAccessDescriptions < 'a , Op > = < Op as Asn1Operation > :: SequenceOfVec < 'a , AccessDescription < 'a > > ;
};
}
