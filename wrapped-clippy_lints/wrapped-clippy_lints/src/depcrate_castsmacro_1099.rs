// Generated macro for macro_1099 (macro)
macro_rules! Depcrate_castsmacro_1099 {
() => {
// Module: crate::casts
// Provides: {"macro_1099"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Catch casts from `0` to some pointer type"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This generally means `null` and is better expressed as"] # [doc = " {`std`, `core`}`::ptr::`{`null`, `null_mut`}."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let a = 0 as *const u32;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let a = std::ptr::null::<u32>();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ZERO_PTR , style , "using `0 as *{const, mut} T`" }
};
}
