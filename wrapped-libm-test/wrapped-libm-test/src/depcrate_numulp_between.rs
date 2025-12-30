// Generated macro for ulp_between (function)
macro_rules! Depcrate_numulp_between {
() => {
// Module: crate::num
// Provides: {"ulp_between"}
// Dependencies: {}
# [doc = " Return the number of steps between two floats, returning `None` if either input is NaN."] # [doc = ""] # [doc = " This is the number of steps needed for `n_up` or `n_down` to go between values. Infinities"] # [doc = " are treated the same as those functions (will return the nearest finite value), and only one"] # [doc = " of `-0` or `+0` is counted. It does not matter which value is greater."] pub fn ulp_between < F : Float > (x : F , y : F) -> Option < F :: Int > { let a = as_ulp_steps (x) ? ; let b = as_ulp_steps (y) ? ; Some (a . abs_diff (b)) }
};
}
