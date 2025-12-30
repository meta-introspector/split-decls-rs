// Generated macro for Normalized3by2Divisor (struct)
macro_rules! Depcrate_barrettNormalized3by2Divisor {
() => {
// Module: crate::barrett
// Provides: {"Normalized3by2Divisor"}
// Dependencies: {}
# [doc = " Divide a 3-Word by a prearranged DoubleWord divisor."] # [doc = ""] # [doc = " Assumes quotient fits in a Word."] # [doc = ""] # [doc = " Möller, Granlund, \"Improved division by invariant integers\""] # [doc = " Algorithm 5."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Normalized3by2Divisor < T , D > { divisor : D , m : T , }
};
}
