// Generated macro for CheckedSub (trait)
macro_rules! Depcrate_traitsCheckedSub {
() => {
// Module: crate::traits
// Provides: {"CheckedSub"}
// Dependencies: {}
# [doc = " Checked subtraction."] pub trait CheckedSub < Rhs = Self > : Sized { # [doc = " Perform checked subtraction, returning a [`CtOption`] which `is_some`"] # [doc = " only if the operation did not underflow."] fn checked_sub (& self , rhs : & Rhs) -> CtOption < Self > ; }
};
}
