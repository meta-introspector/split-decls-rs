// Generated macro for WrappingAdd (trait)
macro_rules! Depcrate_ops_wrappingWrappingAdd {
() => {
// Module: crate::ops::wrapping
// Provides: {"WrappingAdd"}
// Dependencies: {}
# [doc = " Performs addition that wraps around on overflow."] pub trait WrappingAdd : Sized + Add < Self , Output = Self > { # [doc = " Wrapping (modular) addition. Computes `self + other`, wrapping around at the boundary of"] # [doc = " the type."] fn wrapping_add (& self , v : & Self) -> Self ; }
};
}
