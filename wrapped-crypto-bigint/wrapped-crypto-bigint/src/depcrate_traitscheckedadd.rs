// Generated macro for CheckedAdd (trait)
macro_rules! Depcrate_traitsCheckedAdd {
() => {
// Module: crate::traits
// Provides: {"CheckedAdd"}
// Dependencies: {}
# [doc = " Checked addition."] pub trait CheckedAdd < Rhs = Self > : Sized { # [doc = " Perform checked addition, returning a [`CtOption`] which `is_some` only if the operation"] # [doc = " did not overflow."] fn checked_add (& self , rhs : & Rhs) -> CtOption < Self > ; }
};
}
