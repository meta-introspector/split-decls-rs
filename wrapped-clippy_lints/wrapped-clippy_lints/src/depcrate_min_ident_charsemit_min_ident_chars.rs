// Generated macro for emit_min_ident_chars (function)
macro_rules! Depcrate_min_ident_charsemit_min_ident_chars {
() => {
// Module: crate::min_ident_chars
// Provides: {"emit_min_ident_chars"}
// Dependencies: {}
fn emit_min_ident_chars (conf : & MinIdentChars , cx : & impl LintContext , ident : & str , span : Span) { let help = if conf . min_ident_chars_threshold == 1 { Cow :: Borrowed ("this ident consists of a single char") } else { Cow :: Owned (format ! ("this ident is too short ({} <= {})" , ident . len () , conf . min_ident_chars_threshold ,)) } ; span_lint (cx , MIN_IDENT_CHARS , span , help) ; }
};
}
