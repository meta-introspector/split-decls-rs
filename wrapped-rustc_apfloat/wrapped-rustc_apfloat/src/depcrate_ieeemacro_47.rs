// Generated macro for macro_47 (macro)
macro_rules! Depcrate_ieeemacro_47 {
() => {
// Module: crate::ieee
// Provides: {"macro_47"}
// Dependencies: {}
ieee_semantics ! { # [doc = " IEEE binary16 half-precision (16-bit) floating point number."] Half = HalfS (16 : 5) , # [doc = " IEEE binary32 single-precision (32-bit) floating point number."] Single = SingleS (32 : 8) , # [doc = " IEEE binary64 double-precision (64-bit) floating point number."] Double = DoubleS (64 : 11) , # [doc = " IEEE binary128 quadruple-precision (128-bit) floating point number."] Quad = QuadS (128 : 15) , # [doc = " 16-bit brain floating point number."] # [doc = ""] # [doc = " This is not an IEEE kind but uses the same semantics."] BFloat = BFloatS (16 : 8) , # [doc = " 8-bit floating point number with S1E5M2 bit layout."] # [doc = ""] # [doc = " Follows IEEE-754 conventions with S1E5M2 bit layout as described in"] # [doc = " <https://arxiv.org/abs/2209.05433>."] Float8E5M2 = Float8E5M2S (8 : 5) , # [doc = " 8-bit floating point number with S1E4M3 bit layout."] # [doc = ""] # [doc = " This type mostly follows IEEE-754 conventions with a"] # [doc = " bit layout S1E4M3 as described in <https://arxiv.org/abs/2209.05433>."] # [doc = " Unlike IEEE-754 types, there are no infinity values, and NaN is"] # [doc = " represented with the exponent and mantissa bits set to all 1s."] Float8E4M3FN = Float8E4M3FNS (8 : 4) { const NONFINITE_BEHAVIOR : NonfiniteBehavior = NonfiniteBehavior :: NanOnly ; } , }
};
}
