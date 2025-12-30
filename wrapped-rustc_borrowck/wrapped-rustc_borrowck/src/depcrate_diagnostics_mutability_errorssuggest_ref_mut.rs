// Generated macro for suggest_ref_mut (function)
macro_rules! Depcrate_diagnostics_mutability_errorssuggest_ref_mut {
() => {
// Module: crate::diagnostics::mutability_errors
// Provides: {"suggest_ref_mut"}
// Dependencies: {}
# [doc = " If possible, suggest replacing `ref` with `ref mut`."] fn suggest_ref_mut (tcx : TyCtxt < '_ > , span : Span) -> Option < Span > { let pattern_str = tcx . sess . source_map () . span_to_snippet (span) . ok () ? ; if let Some (rest) = pattern_str . strip_prefix ("ref") && rest . starts_with (rustc_lexer :: is_whitespace) { let span = span . with_lo (span . lo () + BytePos (4)) . shrink_to_lo () ; Some (span) } else { None } }
};
}
