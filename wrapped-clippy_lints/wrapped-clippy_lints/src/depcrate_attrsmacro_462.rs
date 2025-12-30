// Generated macro for macro_462 (macro)
macro_rules! Depcrate_attrsmacro_462 {
() => {
// Module: crate::attrs
// Provides: {"macro_462"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for attributes that appear two or more times."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Repeating an attribute on the same item (or globally on the same crate)"] # [doc = " is unnecessary and doesn't have an effect."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[allow(dead_code)]"] # [doc = " #[allow(dead_code)]"] # [doc = " fn foo() {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[allow(dead_code)]"] # [doc = " fn foo() {}"] # [doc = " ```"] # [clippy :: version = "1.79.0"] pub DUPLICATED_ATTRIBUTES , suspicious , "duplicated attribute" }
};
}
