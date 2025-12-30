// Generated macro for parse_macro_expansion_error (function)
macro_rules! Depcrate_dbparse_macro_expansion_error {
() => {
// Module: crate::db
// Provides: {"parse_macro_expansion_error"}
// Dependencies: {}
fn parse_macro_expansion_error (db : & dyn ExpandDatabase , macro_call_id : MacroCallId ,) -> Option < Arc < ExpandResult < Arc < [SyntaxError] > > > > { let e : ExpandResult < Arc < [SyntaxError] > > = db . parse_macro_expansion (macro_call_id) . map (| it | Arc :: from (it . 0 . errors ())) ; if e . value . is_empty () && e . err . is_none () { None } else { Some (Arc :: new (e)) } }
};
}
