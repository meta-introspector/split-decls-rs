// Generated macro for macro_1071 (macro)
macro_rules! Depcrate_castsmacro_1071 {
() => {
// Module: crate::casts
// Provides: {"macro_1071"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts of function pointers to something other than `usize`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Casting a function pointer to anything other than `usize`/`isize` is"] # [doc = " not portable across architectures. If the target type is too small the"] # [doc = " address would be truncated, and target types larger than `usize` are"] # [doc = " unnecessary."] # [doc = ""] # [doc = " Casting to `isize` also doesn't make sense, since addresses are never"] # [doc = " signed."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn fun() -> i32 { 1 }"] # [doc = " let _ = fun as i64;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # fn fun() -> i32 { 1 }"] # [doc = " let _ = fun as usize;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub FN_TO_NUMERIC_CAST , style , "casting a function pointer to a numeric type other than `usize`" }
};
}
