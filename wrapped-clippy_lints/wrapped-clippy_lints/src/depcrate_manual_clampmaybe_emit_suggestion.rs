// Generated macro for maybe_emit_suggestion (function)
macro_rules! Depcrate_manual_clampmaybe_emit_suggestion {
() => {
// Module: crate::manual_clamp
// Provides: {"maybe_emit_suggestion"}
// Dependencies: {}
fn maybe_emit_suggestion < 'tcx > (cx : & LateContext < 'tcx > , suggestion : & ClampSuggestion < 'tcx >) { if ! suggestion . min_less_than_max (cx) { return ; } let ClampSuggestion { params : InputMinMax { input , min , max , is_float , } , span , make_assignment , hir_with_ignore_attr , } = suggestion ; let input = Sugg :: hir (cx , input , "..") . maybe_paren () ; let min = Sugg :: hir (cx , min , "..") ; let max = Sugg :: hir (cx , max , "..") ; let semicolon = if make_assignment . is_some () { ";" } else { "" } ; let assignment = if let Some (assignment) = make_assignment { let assignment = Sugg :: hir (cx , assignment , "..") ; format ! ("{assignment} = ") } else { String :: new () } ; let suggestion = format ! ("{assignment}{input}.clamp({min}, {max}){semicolon}") ; let msg = "clamp-like pattern without using clamp function" ; let lint_builder = | d : & mut Diag < '_ , () > | { d . span_suggestion (* span , "replace with clamp" , suggestion , Applicability :: MaybeIncorrect) ; if * is_float { d . note ("clamp will panic if max < min, min.is_nan(), or max.is_nan()") . note ("clamp returns NaN if the input is NaN") ; } else { d . note ("clamp will panic if max < min") ; } } ; if let Some (hir_id) = hir_with_ignore_attr { span_lint_hir_and_then (cx , MANUAL_CLAMP , * hir_id , * span , msg , lint_builder) ; } else { span_lint_and_then (cx , MANUAL_CLAMP , * span , msg , lint_builder) ; } }
};
}
