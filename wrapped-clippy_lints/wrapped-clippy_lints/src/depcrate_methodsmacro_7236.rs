// Generated macro for macro_7236 (macro)
macro_rules! Depcrate_methodsmacro_7236 {
() => {
// Module: crate::methods
// Provides: {"macro_7236"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Converts some constructs mapping an Enum value for equality comparison."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Calls such as `opt.map_or(false, |val| val == 5)` are needlessly long and cumbersome,"] # [doc = " and can be reduced to, for example, `opt == Some(5)` assuming `opt` implements `PartialEq`."] # [doc = " Also, calls such as `opt.map_or(true, |val| val == 5)` can be reduced to"] # [doc = " `opt.is_none_or(|val| val == 5)`."] # [doc = " This lint offers readability and conciseness improvements."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " pub fn a(x: Option<i32>) -> (bool, bool) {"] # [doc = "     ("] # [doc = "         x.map_or(false, |n| n == 5),"] # [doc = "         x.map_or(true, |n| n > 5),"] # [doc = "     )"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " pub fn a(x: Option<i32>) -> (bool, bool) {"] # [doc = "     ("] # [doc = "         x == Some(5),"] # [doc = "         x.is_none_or(|n| n > 5),"] # [doc = "     )"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.84.0"] pub UNNECESSARY_MAP_OR , style , "reduce unnecessary calls to `.map_or(bool, …)`" }
};
}
