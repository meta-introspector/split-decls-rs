// Generated macro for macro_7241 (macro)
macro_rules! Depcrate_methodsmacro_7241 {
() => {
// Module: crate::methods
// Provides: {"macro_7241"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.map(…)`, followed by `.all(identity)` or `.any(identity)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `.all(…)` or `.any(…)` methods can be called directly in place of `.map(…)`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```"] # [doc = " # let mut v = [\"\"];"] # [doc = " let e1 = v.iter().map(|s| s.is_empty()).all(|a| a);"] # [doc = " let e2 = v.iter().map(|s| s.is_empty()).any(std::convert::identity);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```"] # [doc = " # let mut v = [\"\"];"] # [doc = " let e1 = v.iter().all(|s| s.is_empty());"] # [doc = " let e2 = v.iter().any(|s| s.is_empty());"] # [doc = " ```"] # [clippy :: version = "1.84.0"] pub MAP_ALL_ANY_IDENTITY , complexity , "combine `.map(_)` followed by `.all(identity)`/`.any(identity)` into a single call" }
};
}
