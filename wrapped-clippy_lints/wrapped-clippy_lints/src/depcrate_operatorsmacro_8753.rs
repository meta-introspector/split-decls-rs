// Generated macro for macro_8753 (macro)
macro_rules! Depcrate_operatorsmacro_8753 {
() => {
// Module: crate::operators
// Provides: {"macro_8753"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for modulo arithmetic."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " The results of modulo (`%`) operation might differ"] # [doc = " depending on the language, when negative numbers are involved."] # [doc = " If you interop with different languages it might be beneficial"] # [doc = " to double check all places that use modulo arithmetic."] # [doc = ""] # [doc = " For example, in Rust `17 % -3 = 2`, but in Python `17 % -3 = -1`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = -17 % 3;"] # [doc = " ```"] # [clippy :: version = "1.42.0"] pub MODULO_ARITHMETIC , restriction , "any modulo arithmetic statement" }
};
}
