// Generated macro for detect_option_if_let_else (function)
macro_rules! Depcrate_option_if_let_elsedetect_option_if_let_else {
() => {
// Module: crate::option_if_let_else
// Provides: {"detect_option_if_let_else"}
// Dependencies: {}
# [doc = " If this expression is the option if let/else construct we're detecting, then"] # [doc = " this function returns an `OptionOccurrence` struct with details if"] # [doc = " this construct is found, or None if this construct is not found."] fn detect_option_if_let_else < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) -> Option < OptionOccurrence > { if let Some (higher :: IfLet { let_pat , let_expr , if_then , if_else : Some (if_else) , .. }) = higher :: IfLet :: hir (cx , expr) && ! cx . typeck_results () . expr_ty (expr) . is_unit () && ! is_else_clause (cx . tcx , expr) { try_get_option_occurrence (cx , expr . span . ctxt () , let_pat , let_expr , if_then , if_else) } else { None } }
};
}
