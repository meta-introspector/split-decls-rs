// Generated macro for Position (struct)
macro_rules! Depcrate_with_positionPosition {
() => {
// Module: crate::with_position
// Provides: {"Position"}
// Dependencies: {}
# [doc = " The first component of the value yielded by `WithPosition`."] # [doc = " Indicates the position of this element in the iterator results."] # [doc = ""] # [doc = " See [`.with_position()`](crate::Itertools::with_position) for more information."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct Position { # [doc = " This is the initial element (also true if there's exactly one element)"] pub is_first : bool , # [doc = " This is the final element (also true if there's exactly one element)"] pub is_last : bool , }
};
}
