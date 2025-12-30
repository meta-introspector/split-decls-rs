// Generated macro for macro_158 (macro)
macro_rules! Depcrate_symbolsmacro_158 {
() => {
// Module: crate::symbols
// Provides: {"macro_158"}
// Dependencies: {}
declare_tool_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `Symbol::as_str`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's faster and easier to use the symbol constant. If one doesn't exist it can be added to `clippy_utils/src/sym.rs`"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " symbol.as_str() == \"foo\""] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " symbol == sym::foo"] # [doc = " ```"] pub clippy :: SYMBOL_AS_STR , Warn , "calls to `Symbol::as_str`" , report_in_external_macro : true }
};
}
