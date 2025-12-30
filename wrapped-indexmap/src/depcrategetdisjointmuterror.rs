// Generated macro for GetDisjointMutError (enum)
macro_rules! DepcrateGetDisjointMutError {
() => {
// Module: crate
// Provides: {"GetDisjointMutError"}
// Dependencies: {}
# [doc = " The error type returned by [`get_disjoint_indices_mut`][`IndexMap::get_disjoint_indices_mut`]."] # [doc = ""] # [doc = " It indicates one of two possible errors:"] # [doc = " - An index is out-of-bounds."] # [doc = " - The same index appeared multiple times in the array."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum GetDisjointMutError { # [doc = " An index provided was out-of-bounds for the slice."] IndexOutOfBounds , # [doc = " Two indices provided were overlapping."] OverlappingIndices , }
};
}
