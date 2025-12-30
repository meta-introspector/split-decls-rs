// Generated macro for signed_float_impl (macro)
macro_rules! Depcrate_signsigned_float_impl {
() => {
// Module: crate::sign
// Provides: {"signed_float_impl"}
// Dependencies: {}
macro_rules ! signed_float_impl { ($ t : ty) => { impl Signed for $ t { # [doc = " Computes the absolute value. Returns `NAN` if the number is `NAN`."] # [inline] fn abs (& self) -> $ t { FloatCore :: abs (* self) } # [doc = " The positive difference of two numbers. Returns `0.0` if the number is"] # [doc = " less than or equal to `other`, otherwise the difference between`self`"] # [doc = " and `other` is returned."] # [inline] fn abs_sub (& self , other : &$ t) -> $ t { if * self <= * other { 0. } else { * self - * other } } # [doc = " # Returns"] # [doc = ""] # [doc = " - `1.0` if the number is positive, `+0.0` or `INFINITY`"] # [doc = " - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`"] # [doc = " - `NAN` if the number is NaN"] # [inline] fn signum (& self) -> $ t { FloatCore :: signum (* self) } # [doc = " Returns `true` if the number is positive, including `+0.0` and `INFINITY`"] # [inline] fn is_positive (& self) -> bool { FloatCore :: is_sign_positive (* self) } # [doc = " Returns `true` if the number is negative, including `-0.0` and `NEG_INFINITY`"] # [inline] fn is_negative (& self) -> bool { FloatCore :: is_sign_negative (* self) } } } ; }
};
}
