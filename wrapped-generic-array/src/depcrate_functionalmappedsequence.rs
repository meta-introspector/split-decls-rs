// Generated macro for MappedSequence (type)
macro_rules! Depcrate_functionalMappedSequence {
() => {
// Module: crate::functional
// Provides: {"MappedSequence"}
// Dependencies: {}
# [doc = " Accessor type for a mapped generic sequence"] # [doc = ""] # [doc = " NOTE: The choice to use the `Sequence` associated type here instead of `Mapped`"] # [doc = " is due to only the `Sequence` type being guaranteed to implement `FromIterator`."] # [doc = " However, this does lead to some oddity where `FallibleGenericSequence::from_fallible_iter`"] # [doc = " is implemented on `Mapped`, but returns the `Sequence`/`MappedSequence` type. Same difference, though."] pub type MappedSequence < S , T , U > = < Mapped < S , T , U > as GenericSequence < U > > :: Sequence ;
};
}
