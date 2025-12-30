// Generated macro for OverflowingMul (trait)
macro_rules! Depcrate_ops_overflowingOverflowingMul {
() => {
// Module: crate::ops::overflowing
// Provides: {"OverflowingMul"}
// Dependencies: {}
# [doc = " Performs multiplication with a flag for overflow."] pub trait OverflowingMul : Sized + Mul < Self , Output = Self > { # [doc = " Returns a tuple of the product along with a boolean indicating whether an arithmetic overflow would occur."] # [doc = " If an overflow would have occurred then the wrapped value is returned."] fn overflowing_mul (& self , v : & Self) -> (Self , bool) ; }
};
}
