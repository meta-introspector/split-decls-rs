// Generated macro for emit_lint (function)
macro_rules! Depcrate_methods_unnecessary_result_map_or_elseemit_lint {
() => {
// Module: crate::methods::unnecessary_result_map_or_else
// Provides: {"emit_lint"}
// Dependencies: {}
fn emit_lint (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , def_arg : & Expr < '_ >) { let msg = "unused \"map closure\" when calling `Result::map_or_else` value" ; let self_snippet = snippet (cx , recv . span , "..") ; let err_snippet = snippet (cx , def_arg . span , "..") ; span_lint_and_sugg (cx , UNNECESSARY_RESULT_MAP_OR_ELSE , expr . span , msg , "consider using `unwrap_or_else`" , format ! ("{self_snippet}.unwrap_or_else({err_snippet})") , Applicability :: MachineApplicable ,) ; }
};
}
