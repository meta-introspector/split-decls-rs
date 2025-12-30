// Generated macro for impl_4073 (impl)
macro_rules! Depcrate_manual_clampimpl_4073 {
() => {
// Module: crate::manual_clamp
// Provides: {"impl_4073"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ManualClamp { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if ! expr . span . from_expansion () && ! is_in_const_context (cx) { let suggestion = is_if_elseif_else_pattern (cx , expr) . or_else (| | is_max_min_pattern (cx , expr)) . or_else (| | is_call_max_min_pattern (cx , expr)) . or_else (| | is_match_pattern (cx , expr)) . or_else (| | is_if_elseif_pattern (cx , expr)) ; if let Some (suggestion) = suggestion && self . msrv . meets (cx , msrvs :: CLAMP) { maybe_emit_suggestion (cx , & suggestion) ; } } } fn check_block (& mut self , cx : & LateContext < 'tcx > , block : & 'tcx Block < 'tcx >) { if is_in_const_context (cx) || ! self . msrv . meets (cx , msrvs :: CLAMP) { return ; } for suggestion in is_two_if_pattern (cx , block) { maybe_emit_suggestion (cx , & suggestion) ; } } }
};
}
