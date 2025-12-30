// Generated macro for macro_1085 (macro)
macro_rules! Depcrate_castsmacro_1085 {
() => {
// Module: crate::casts
// Provides: {"macro_1085"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts of a function pointer to a numeric type not wide enough to"] # [doc = " store an address."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Such a cast discards some bits of the function's address. If this is intended, it would be more"] # [doc = " clearly expressed by casting to `usize` first, then casting the `usize` to the intended type (with"] # [doc = " a comment) to perform the truncation."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn fn1() -> i16 {"] # [doc = "     1"] # [doc = " };"] # [doc = " let _ = fn1 as i32;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " // Cast to usize first, then comment with the reason for the truncation"] # [doc = " fn fn1() -> i16 {"] # [doc = "     1"] # [doc = " };"] # [doc = " let fn_ptr = fn1 as usize;"] # [doc = " let fn_ptr_truncated = fn_ptr as i32;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub FN_TO_NUMERIC_CAST_WITH_TRUNCATION , style , "casting a function pointer to a numeric type not wide enough to store the address" }
};
}
