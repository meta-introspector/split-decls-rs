// Generated macro for format_err (macro)
macro_rules! Depcrate_macrosformat_err {
() => {
// Module: crate::macros
// Provides: {"format_err"}
// Dependencies: {}
macro_rules ! format_err { ($ obj : expr , $ ($ format : tt) +) => { { # [allow (unused_imports)] use $ crate :: utils :: error ::*; let msg = format ! ($ ($ format) +) ; $ obj . EXPECTED_Span_OR_ToTokens (msg) } } ; }
};
}
