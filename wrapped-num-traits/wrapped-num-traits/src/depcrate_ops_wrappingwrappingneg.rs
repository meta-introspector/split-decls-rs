// Generated macro for WrappingNeg (trait)
macro_rules! Depcrate_ops_wrappingWrappingNeg {
() => {
// Module: crate::ops::wrapping
// Provides: {"WrappingNeg"}
// Dependencies: {}
# [doc = " Performs a negation that does not panic."] pub trait WrappingNeg : Sized { # [doc = " Wrapping (modular) negation. Computes `-self`,"] # [doc = " wrapping around at the boundary of the type."] # [doc = ""] # [doc = " Since unsigned types do not have negative equivalents"] # [doc = " all applications of this function will wrap (except for `-0`)."] # [doc = " For values smaller than the corresponding signed type's maximum"] # [doc = " the result is the same as casting the corresponding signed value."] # [doc = " Any larger values are equivalent to `MAX + 1 - (val - MAX - 1)` where"] # [doc = " `MAX` is the corresponding signed type's maximum."] # [doc = ""] # [doc = " ```"] # [doc = " use num_traits::WrappingNeg;"] # [doc = ""] # [doc = " assert_eq!(100i8.wrapping_neg(), -100);"] # [doc = " assert_eq!((-100i8).wrapping_neg(), 100);"] # [doc = " assert_eq!((-128i8).wrapping_neg(), -128); // wrapped!"] # [doc = " ```"] fn wrapping_neg (& self) -> Self ; }
};
}
