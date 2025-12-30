// Generated macro for SequenceOfGeneralName (type)
macro_rules! Depcrate_nameSequenceOfGeneralName {
() => {
// Module: crate::name
// Provides: {"SequenceOfGeneralName"}
// Dependencies: {}
pub (crate) type SequenceOfGeneralName < 'a , Op > = < Op as Asn1Operation > :: SequenceOfVec < 'a , GeneralName < 'a > > ;
};
}
