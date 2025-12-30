// Generated macro for span_with_ctxt_from_mark (function)
macro_rules! Depcrate_hygienespan_with_ctxt_from_mark {
() => {
// Module: crate::hygiene
// Provides: {"span_with_ctxt_from_mark"}
// Dependencies: {}
fn span_with_ctxt_from_mark (db : & dyn ExpandDatabase , span : Span , expn_id : MacroCallId , transparency : Transparency , edition : Edition ,) -> Span { Span { ctx : apply_mark (db , SyntaxContext :: root (edition) , expn_id , transparency , edition) , .. span } }
};
}
