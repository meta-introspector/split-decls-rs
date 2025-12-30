// Generated macro for Signed (trait)
macro_rules! Depcrate_signSigned {
() => {
// Module: crate::sign
// Provides: {"Signed"}
// Dependencies: {}
# [doc = " Useful functions for signed numbers (i.e. numbers that can be negative)."] pub trait Signed : Sized + Num + Neg < Output = Self > { # [doc = " Computes the absolute value."] # [doc = ""] # [doc = " For `f32` and `f64`, `NaN` will be returned if the number is `NaN`."] # [doc = ""] # [doc = " For signed integers, `::MIN` will be returned if the number is `::MIN`."] fn abs (& self) -> Self ; # [doc = " The positive difference of two numbers."] # [doc = ""] # [doc = " Returns `zero` if the number is less than or equal to `other`, otherwise the difference"] # [doc = " between `self` and `other` is returned."] fn abs_sub (& self , other : & Self) -> Self ; # [doc = " Returns the sign of the number."] # [doc = ""] # [doc = " For `f32` and `f64`:"] # [doc = ""] # [doc = " * `1.0` if the number is positive, `+0.0` or `INFINITY`"] # [doc = " * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`"] # [doc = " * `NaN` if the number is `NaN`"] # [doc = ""] # [doc = " For signed integers:"] # [doc = ""] # [doc = " * `0` if the number is zero"] # [doc = " * `1` if the number is positive"] # [doc = " * `-1` if the number is negative"] fn signum (& self) -> Self ; # [doc = " Returns true if the number is positive and false if the number is zero or negative."] fn is_positive (& self) -> bool ; # [doc = " Returns true if the number is negative and false if the number is zero or positive."] fn is_negative (& self) -> bool ; }
};
}
