// Generated macro for CheckedDiv (trait)
macro_rules! Depcrate_traitsCheckedDiv {
() => {
// Module: crate::traits
// Provides: {"CheckedDiv"}
// Dependencies: {}
# [doc = " Checked division."] pub trait CheckedDiv < Rhs = Self > : Sized { # [doc = " Perform checked division, returning a [`CtOption`] which `is_some` only if the divisor is"] # [doc = " non-zero."] fn checked_div (& self , rhs : & Rhs) -> CtOption < Self > ; }
};
}
