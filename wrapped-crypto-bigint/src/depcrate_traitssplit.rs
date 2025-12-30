// Generated macro for Split (trait)
macro_rules! Depcrate_traitsSplit {
() => {
// Module: crate::traits
// Provides: {"Split"}
// Dependencies: {}
# [doc = " Split a number in half, returning the least significant half followed by the most significant."] pub trait Split : SplitMixed < Self :: Output , Self :: Output > { # [doc = " Split output: low/high components of the value."] type Output ; # [doc = " Split this number in half, returning its low and high components respectively."] fn split (& self) -> (Self :: Output , Self :: Output) { self . split_mixed () } }
};
}
