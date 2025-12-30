// Generated macro for ExprToSpannedString (struct)
macro_rules! Depcrate_utilExprToSpannedString {
() => {
// Module: crate::util
// Provides: {"ExprToSpannedString"}
// Dependencies: {}
pub (crate) struct ExprToSpannedString { pub symbol : Symbol , pub style : ast :: StrStyle , pub span : Span , # [doc = " The raw string literal, with no escaping or processing."] # [doc = ""] # [doc = " Generally only useful for lints that care about the raw bytes the user wrote."] pub uncooked_symbol : (ast :: token :: LitKind , Symbol) , }
};
}
