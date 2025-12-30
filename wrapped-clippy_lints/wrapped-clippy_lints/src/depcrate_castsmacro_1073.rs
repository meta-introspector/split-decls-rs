// Generated macro for macro_1073 (macro)
macro_rules! Depcrate_castsmacro_1073 {
() => {
// Module: crate::casts
// Provides: {"macro_1073"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts of a function pointer to any integer type."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Casting a function pointer to an integer can have surprising results and can occur"] # [doc = " accidentally if parentheses are omitted from a function call. If you aren't doing anything"] # [doc = " low-level with function pointers then you can opt out of casting functions to integers in"] # [doc = " order to avoid mistakes. Alternatively, you can use this lint to audit all uses of function"] # [doc = " pointer casts in your code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " // fn1 is cast as `usize`"] # [doc = " fn fn1() -> u16 {"] # [doc = "     1"] # [doc = " };"] # [doc = " let _ = fn1 as usize;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " // maybe you intended to call the function?"] # [doc = " fn fn2() -> u16 {"] # [doc = "     1"] # [doc = " };"] # [doc = " let _ = fn2() as usize;"] # [doc = ""] # [doc = " // or"] # [doc = ""] # [doc = " // maybe you intended to cast it to a function type?"] # [doc = " fn fn3() -> u16 {"] # [doc = "     1"] # [doc = " }"] # [doc = " let _ = fn3 as fn() -> u16;"] # [doc = " ```"] # [clippy :: version = "1.58.0"] pub FN_TO_NUMERIC_CAST_ANY , restriction , "casting a function pointer to any integer type" }
};
}
