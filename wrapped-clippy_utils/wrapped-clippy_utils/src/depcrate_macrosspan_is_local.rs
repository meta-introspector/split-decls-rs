// Generated macro for span_is_local (function)
macro_rules! Depcrate_macrosspan_is_local {
() => {
// Module: crate::macros
// Provides: {"span_is_local"}
// Dependencies: {}
# [doc = " Checks whether the span is from the root expansion or a locally defined macro"] pub fn span_is_local (span : Span) -> bool { ! span . from_expansion () || expn_is_local (span . ctxt () . outer_expn ()) }
};
}
