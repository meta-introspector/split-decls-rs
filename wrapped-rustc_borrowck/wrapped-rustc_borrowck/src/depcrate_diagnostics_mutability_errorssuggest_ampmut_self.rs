// Generated macro for suggest_ampmut_self (function)
macro_rules! Depcrate_diagnostics_mutability_errorssuggest_ampmut_self {
() => {
// Module: crate::diagnostics::mutability_errors
// Provides: {"suggest_ampmut_self"}
// Dependencies: {}
fn suggest_ampmut_self (tcx : TyCtxt < '_ > , span : Span) -> (Span , String) { match tcx . sess . source_map () . span_to_snippet (span) { Ok (snippet) if snippet . ends_with ("self") => { (span . with_hi (span . hi () - BytePos (4)) . shrink_to_hi () , "mut " . to_string ()) } _ => (span , "&mut self" . to_string ()) , } }
};
}
