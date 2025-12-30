// Generated macro for SequenceItem (type)
macro_rules! Depcrate_sequenceSequenceItem {
() => {
// Module: crate::sequence
// Provides: {"SequenceItem"}
// Dependencies: {}
# [doc = " Accessor for `GenericSequence` item type, which is really `IntoIterator::Item`"] # [doc = ""] # [doc = " For deeply nested generic mapped sequence types, like shown in `tests/generics.rs`,"] # [doc = " this can be useful for keeping things organized."] pub type SequenceItem < T > = < T as IntoIterator > :: Item ;
};
}
