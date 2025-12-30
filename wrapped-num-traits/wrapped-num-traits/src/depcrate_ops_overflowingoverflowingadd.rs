// Generated macro for OverflowingAdd (trait)
macro_rules! Depcrate_ops_overflowingOverflowingAdd {
() => {
// Module: crate::ops::overflowing
// Provides: {"OverflowingAdd"}
// Dependencies: {}
# [doc = " Performs addition with a flag for overflow."] pub trait OverflowingAdd : Sized + Add < Self , Output = Self > { # [doc = " Returns a tuple of the sum along with a boolean indicating whether an arithmetic overflow would occur."] # [doc = " If an overflow would have occurred then the wrapped value is returned."] fn overflowing_add (& self , v : & Self) -> (Self , bool) ; }
};
}
