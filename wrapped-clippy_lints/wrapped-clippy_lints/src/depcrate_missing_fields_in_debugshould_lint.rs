// Generated macro for should_lint (function)
macro_rules! Depcrate_missing_fields_in_debugshould_lint {
() => {
// Module: crate::missing_fields_in_debug
// Provides: {"should_lint"}
// Dependencies: {}
# [doc = " Checks if we should lint in a block of code"] # [doc = ""] # [doc = " The way we check for this condition is by checking if there is"] # [doc = " a call to `Formatter::debug_struct` but no call to `.finish_non_exhaustive()`."] fn should_lint < 'tcx > (cx : & LateContext < 'tcx > , typeck_results : & TypeckResults < 'tcx > , block : impl Visitable < 'tcx > ,) -> bool { let mut has_finish_non_exhaustive = false ; let mut has_debug_struct = false ; for_each_expr (cx , block , | expr | { if let ExprKind :: MethodCall (path , recv , ..) = & expr . kind { let recv_ty = typeck_results . expr_ty (recv) . peel_refs () ; if path . ident . name == sym :: debug_struct && recv_ty . is_diag_item (cx , sym :: Formatter) { has_debug_struct = true ; } else if path . ident . name == sym :: finish_non_exhaustive && recv_ty . is_diag_item (cx , sym :: DebugStruct) { has_finish_non_exhaustive = true ; } } ControlFlow :: < ! , _ > :: Continue (()) }) ; ! has_finish_non_exhaustive && has_debug_struct }
};
}
