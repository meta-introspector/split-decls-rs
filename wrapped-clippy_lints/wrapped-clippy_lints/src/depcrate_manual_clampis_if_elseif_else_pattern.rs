// Generated macro for is_if_elseif_else_pattern (function)
macro_rules! Depcrate_manual_clampis_if_elseif_else_pattern {
() => {
// Module: crate::manual_clamp
// Provides: {"is_if_elseif_else_pattern"}
// Dependencies: {}
# [doc = " Targets patterns like"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let (input, min, max) = (0, -3, 12);"] # [doc = ""] # [doc = " if input < min {"] # [doc = "     min"] # [doc = " } else if input > max {"] # [doc = "     max"] # [doc = " } else {"] # [doc = "     input"] # [doc = " }"] # [doc = " # ;"] # [doc = " ```"] fn is_if_elseif_else_pattern < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> Option < ClampSuggestion < 'tcx > > { if let Some (If { cond , then , r#else : Some (else_if) , }) = If :: hir (expr) && let Some (If { cond : else_if_cond , then : else_if_then , r#else : Some (else_body) , }) = If :: hir (peel_blocks (else_if)) { let params = is_clamp_meta_pattern (cx , & BinaryOp :: new (peel_blocks (cond)) ? , & BinaryOp :: new (peel_blocks (else_if_cond)) ? , peel_blocks (then) , peel_blocks (else_if_then) , None ,) ? ; if ! eq_expr_value (cx , params . input , peel_blocks (else_body)) { return None ; } Some (ClampSuggestion { params , span : expr . span , make_assignment : None , hir_with_ignore_attr : None , }) } else { None } }
};
}
