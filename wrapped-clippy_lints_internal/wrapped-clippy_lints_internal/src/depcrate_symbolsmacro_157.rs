// Generated macro for macro_157 (macro)
macro_rules! Depcrate_symbolsmacro_157 {
() => {
// Module: crate::symbols
// Provides: {"macro_157"}
// Dependencies: {}
declare_tool_lint ! { # [doc = " ### What it does"] # [doc = " Checks for interning string literals as symbols"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's faster and easier to use the symbol constant. If one doesn't exist it can be added to `clippy_utils/src/sym.rs`"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let _ = Symbol::intern(\"f32\");"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " let _ = sym::f32;"] # [doc = " ```"] pub clippy :: INTERNING_LITERALS , Warn , "interning a symbol that is a literal" , report_in_external_macro : true }
};
}
