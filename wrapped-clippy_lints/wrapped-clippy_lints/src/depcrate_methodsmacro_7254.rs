// Generated macro for macro_7254 (macro)
macro_rules! Depcrate_methodsmacro_7254 {
() => {
// Module: crate::methods
// Provides: {"macro_7254"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " Checks for usage of `.map_or_else()` \"map closure\" for `Option` type."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This can be written more concisely by using `unwrap_or_else()`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let k = 10;"] # [doc = " let x: Option<u32> = Some(4);"] # [doc = " let y = x.map_or_else(|| 2 * k, |n| n);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let k = 10;"] # [doc = " let x: Option<u32> = Some(4);"] # [doc = " let y = x.unwrap_or_else(|| 2 * k);"] # [doc = " ```"] # [clippy :: version = "1.88.0"] pub UNNECESSARY_OPTION_MAP_OR_ELSE , suspicious , "making no use of the \"map closure\" when calling `.map_or_else(|| 2 * k, |n| n)`" }
};
}
