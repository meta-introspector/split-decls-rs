// Generated macro for MulAdd (trait)
macro_rules! Depcrate_ops_mul_addMulAdd {
() => {
// Module: crate::ops::mul_add
// Provides: {"MulAdd"}
// Dependencies: {}
# [doc = " Fused multiply-add. Computes `(self * a) + b` with only one rounding"] # [doc = " error, yielding a more accurate result than an unfused multiply-add."] # [doc = ""] # [doc = " Using `mul_add` can be more performant than an unfused multiply-add if"] # [doc = " the target architecture has a dedicated `fma` CPU instruction."] # [doc = ""] # [doc = " Note that `A` and `B` are `Self` by default, but this is not mandatory."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::f32;"] # [doc = ""] # [doc = " let m = 10.0_f32;"] # [doc = " let x = 4.0_f32;"] # [doc = " let b = 60.0_f32;"] # [doc = ""] # [doc = " // 100.0"] # [doc = " let abs_difference = (m.mul_add(x, b) - (m*x + b)).abs();"] # [doc = ""] # [doc = " assert!(abs_difference <= 100.0 * f32::EPSILON);"] # [doc = " ```"] pub trait MulAdd < A = Self , B = Self > { # [doc = " The resulting type after applying the fused multiply-add."] type Output ; # [doc = " Performs the fused multiply-add operation `(self * a) + b`"] fn mul_add (self , a : A , b : B) -> Self :: Output ; }
};
}
