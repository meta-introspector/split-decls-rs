// Generated macro for macro_1103 (macro)
macro_rules! Depcrate_castsmacro_1103 {
() => {
// Module: crate::casts
// Provides: {"macro_1103"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts of a primitive method pointer like `max`/`min` to any integer type."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Casting a function pointer to an integer can have surprising results and can occur"] # [doc = " accidentally if parentheses are omitted from a function call. If you aren't doing anything"] # [doc = " low-level with function pointers then you can opt out of casting functions to integers in"] # [doc = " order to avoid mistakes. Alternatively, you can use this lint to audit all uses of function"] # [doc = " pointer casts in your code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _ = u16::max as usize;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let _ = u16::MAX as usize;"] # [doc = " ```"] # [clippy :: version = "1.89.0"] pub CONFUSING_METHOD_TO_NUMERIC_CAST , suspicious , "casting a primitive method pointer to any integer type" }
};
}
