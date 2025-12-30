// Generated macro for signum (function)
macro_rules! Depcrate_signsignum {
() => {
// Module: crate::sign
// Provides: {"signum"}
// Dependencies: {}
# [doc = " Returns the sign of the number."] # [doc = ""] # [doc = " For `f32` and `f64`:"] # [doc = ""] # [doc = " * `1.0` if the number is positive, `+0.0` or `INFINITY`"] # [doc = " * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`"] # [doc = " * `NaN` if the number is `NaN`"] # [doc = ""] # [doc = " For signed integers:"] # [doc = ""] # [doc = " * `0` if the number is zero"] # [doc = " * `1` if the number is positive"] # [doc = " * `-1` if the number is negative"] # [inline (always)] pub fn signum < T : Signed > (value : T) -> T { value . signum () }
};
}
