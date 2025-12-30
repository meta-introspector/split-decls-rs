// Generated macro for emit_invalid_type (function)
macro_rules! Depcrate_await_holding_invalidemit_invalid_type {
() => {
// Module: crate::await_holding_invalid
// Provides: {"emit_invalid_type"}
// Dependencies: {}
fn emit_invalid_type (cx : & LateContext < '_ > , span : Span , path : & 'static str , disallowed_path : & 'static DisallowedPathWithoutReplacement ,) { span_lint_and_then (cx , AWAIT_HOLDING_INVALID_TYPE , span , format ! ("holding a disallowed type across an await point `{path}`") , disallowed_path . diag_amendment (span) ,) ; }
};
}
