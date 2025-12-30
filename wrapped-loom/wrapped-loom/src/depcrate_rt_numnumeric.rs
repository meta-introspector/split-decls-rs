// Generated macro for Numeric (trait)
macro_rules! Depcrate_rt_numNumeric {
() => {
// Module: crate::rt::num
// Provides: {"Numeric"}
// Dependencies: {}
# [doc = " Numeric-like type can be represented by a `u64`."] # [doc = ""] # [doc = " Used by `Atomic` to store values."] pub (crate) trait Numeric : Sized + Copy + PartialEq { # [doc = " Convert a value into `u64` representation"] fn into_u64 (self) -> u64 ; # [doc = " Convert a `u64` representation into the value"] fn from_u64 (src : u64) -> Self ; }
};
}
