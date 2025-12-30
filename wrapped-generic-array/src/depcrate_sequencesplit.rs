// Generated macro for Split (trait)
macro_rules! Depcrate_sequenceSplit {
() => {
// Module: crate::sequence
// Provides: {"Split"}
// Dependencies: {}
# [doc = " Defines a `GenericSequence` that can be split into two parts at a given pivot index."] # [doc = ""] # [doc = " # Safety"] # [doc = " While the [`split`](Split::split) method is marked safe,"] # [doc = " care must be taken when implementing it."] pub unsafe trait Split < T , K : ArrayLength > : GenericSequence < T > { # [doc = " First part of the resulting split array"] type First : GenericSequence < T > ; # [doc = " Second part of the resulting split array"] type Second : GenericSequence < T > ; # [doc = " Splits an array at the given index, returning the separate parts of the array."] fn split (self) -> (Self :: First , Self :: Second) ; }
};
}
