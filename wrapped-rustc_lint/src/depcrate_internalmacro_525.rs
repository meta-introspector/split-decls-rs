// Generated macro for macro_525 (macro)
macro_rules! Depcrate_internalmacro_525 {
() => {
// Module: crate::internal
// Provides: {"macro_525"}
// Dependencies: {}
declare_tool_lint ! { # [doc = " The `symbol_intern_string_literal` detects `Symbol::intern` being called on a string literal"] pub rustc :: SYMBOL_INTERN_STRING_LITERAL , Allow , "Forbid uses of string literals in `Symbol::intern`, suggesting preinterning instead" , report_in_external_macro : true }
};
}
