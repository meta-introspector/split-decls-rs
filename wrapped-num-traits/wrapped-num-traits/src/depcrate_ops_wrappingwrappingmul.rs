// Generated macro for WrappingMul (trait)
macro_rules! Depcrate_ops_wrappingWrappingMul {
() => {
// Module: crate::ops::wrapping
// Provides: {"WrappingMul"}
// Dependencies: {}
# [doc = " Performs multiplication that wraps around on overflow."] pub trait WrappingMul : Sized + Mul < Self , Output = Self > { # [doc = " Wrapping (modular) multiplication. Computes `self * other`, wrapping around at the boundary"] # [doc = " of the type."] fn wrapping_mul (& self , v : & Self) -> Self ; }
};
}
