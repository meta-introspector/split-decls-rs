// Generated macro for note_span (macro)
macro_rules! Depcrate_diagnosticsnote_span {
() => {
// Module: crate::diagnostics
// Provides: {"note_span"}
// Dependencies: {}
# [doc = " Generate a note/help text with a span."] macro_rules ! note_span { ($ span : expr , $ ($ tt : tt) *) => { (Some ($ span) , format ! ($ ($ tt) *)) } ; }
};
}
