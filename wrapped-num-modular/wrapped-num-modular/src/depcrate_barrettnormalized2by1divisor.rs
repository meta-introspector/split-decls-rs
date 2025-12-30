// Generated macro for Normalized2by1Divisor (struct)
macro_rules! Depcrate_barrettNormalized2by1Divisor {
() => {
// Module: crate::barrett
// Provides: {"Normalized2by1Divisor"}
// Dependencies: {}
# [doc = " Divide a DoubleWord by a prearranged divisor."] # [doc = ""] # [doc = " Assumes quotient fits in a Word."] # [doc = ""] # [doc = " Möller, Granlund, \"Improved division by invariant integers\", Algorithm 4."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Normalized2by1Divisor < T > { divisor : T , m : T , }
};
}
