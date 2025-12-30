// Generated macro for OverflowingSub (trait)
macro_rules! Depcrate_ops_overflowingOverflowingSub {
() => {
// Module: crate::ops::overflowing
// Provides: {"OverflowingSub"}
// Dependencies: {}
# [doc = " Performs substraction with a flag for overflow."] pub trait OverflowingSub : Sized + Sub < Self , Output = Self > { # [doc = " Returns a tuple of the difference along with a boolean indicating whether an arithmetic overflow would occur."] # [doc = " If an overflow would have occurred then the wrapped value is returned."] fn overflowing_sub (& self , v : & Self) -> (Self , bool) ; }
};
}
