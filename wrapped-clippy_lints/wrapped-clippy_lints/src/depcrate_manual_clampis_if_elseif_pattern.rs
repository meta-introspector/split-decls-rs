// Generated macro for is_if_elseif_pattern (function)
macro_rules! Depcrate_manual_clampis_if_elseif_pattern {
() => {
// Module: crate::manual_clamp
// Provides: {"is_if_elseif_pattern"}
// Dependencies: {}
# [doc = " Targets patterns like"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let (mut input, min, max) = (0, -3, 12);"] # [doc = ""] # [doc = " if input < min {"] # [doc = "     input = min;"] # [doc = " } else if input > max {"] # [doc = "     input = max;"] # [doc = " }"] # [doc = " ```"] fn is_if_elseif_pattern < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> Option < ClampSuggestion < 'tcx > > { if let Some (If { cond , then , r#else : Some (else_if) , }) = If :: hir (expr) && let Some (If { cond : else_if_cond , then : else_if_then , r#else : None , }) = If :: hir (peel_blocks (else_if)) && let ExprKind :: Assign (maybe_input_first_path , maybe_min_max_first , _) = peel_blocks_with_stmt (then) . kind && let ExprKind :: Assign (maybe_input_second_path , maybe_min_max_second , _) = peel_blocks_with_stmt (else_if_then) . kind { let params = is_clamp_meta_pattern (cx , & BinaryOp :: new (peel_blocks (cond)) ? , & BinaryOp :: new (peel_blocks (else_if_cond)) ? , peel_blocks (maybe_min_max_first) , peel_blocks (maybe_min_max_second) , None ,) ? ; if ! eq_expr_value (cx , maybe_input_first_path , maybe_input_second_path) { return None ; } Some (ClampSuggestion { params , span : expr . span , make_assignment : Some (maybe_input_first_path) , hir_with_ignore_attr : None , }) } else { None } }
};
}
