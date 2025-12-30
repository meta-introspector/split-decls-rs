// Generated macro for WrappingSub (trait)
macro_rules! Depcrate_ops_wrappingWrappingSub {
() => {
// Module: crate::ops::wrapping
// Provides: {"WrappingSub"}
// Dependencies: {}
# [doc = " Performs subtraction that wraps around on overflow."] pub trait WrappingSub : Sized + Sub < Self , Output = Self > { # [doc = " Wrapping (modular) subtraction. Computes `self - other`, wrapping around at the boundary"] # [doc = " of the type."] fn wrapping_sub (& self , v : & Self) -> Self ; }
};
}
