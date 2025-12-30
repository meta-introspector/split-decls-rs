// Generated macro for macro_10091 (macro)
macro_rules! Depcrate_swapmacro_10091 {
() => {
// Module: crate::swap
// Provides: {"macro_10091"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `foo = bar; bar = foo` sequences."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This looks like a failed attempt to swap."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let mut a = 1;"] # [doc = " # let mut b = 2;"] # [doc = " a = b;"] # [doc = " b = a;"] # [doc = " ```"] # [doc = " If swapping is intended, use `swap()` instead:"] # [doc = " ```no_run"] # [doc = " # let mut a = 1;"] # [doc = " # let mut b = 2;"] # [doc = " std::mem::swap(&mut a, &mut b);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ALMOST_SWAPPED , correctness , "`foo = bar; bar = foo` sequence" }
};
}
