// Generated macro for emit_redundant_guards (function)
macro_rules! Depcrate_matches_redundant_guardsemit_redundant_guards {
() => {
// Module: crate::matches::redundant_guards
// Provides: {"emit_redundant_guards"}
// Dependencies: {}
fn emit_redundant_guards < 'tcx > (cx : & LateContext < 'tcx > , outer_arm : & Arm < 'tcx > , guard_span : Span , binding_replacement : Cow < 'static , str > , pat_binding : & PatBindingInfo , inner_guard : Option < & Expr < '_ > > ,) { span_lint_and_then (cx , REDUNDANT_GUARDS , guard_span . source_callsite () , "redundant guard" , | diag | { let suggestion_span = match * pat_binding { PatBindingInfo { span , byref_ident : Some (ident) , is_field : true , } => (span , format ! ("{ident}: {binding_replacement}")) , PatBindingInfo { span , is_field : true , .. } => (span . shrink_to_hi () , format ! (": {binding_replacement}")) , PatBindingInfo { span , .. } => (span , binding_replacement . into_owned ()) , } ; diag . multipart_suggestion_verbose ("try" , vec ! [suggestion_span , (guard_span . source_callsite () . with_lo (outer_arm . pat . span . hi ()) , inner_guard . map_or_else (String :: new , | guard | { format ! (" if {}" , snippet (cx , guard . span , "<guard>")) }) ,) ,] , Applicability :: MaybeIncorrect ,) ; } ,) ; }
};
}
