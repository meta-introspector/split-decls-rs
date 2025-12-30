// Generated macro for emit_lint (function)
macro_rules! Depcrate_methods_unnecessary_option_map_or_elseemit_lint {
() => {
// Module: crate::methods::unnecessary_option_map_or_else
// Provides: {"emit_lint"}
// Dependencies: {}
fn emit_lint (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , def_arg : & Expr < '_ >) { let msg = "unused \"map closure\" when calling `Option::map_or_else` value" ; let mut applicability = Applicability :: MachineApplicable ; let self_snippet = snippet_with_applicability (cx , recv . span , "_" , & mut applicability) ; let err_snippet = snippet_with_applicability (cx , def_arg . span , ".." , & mut applicability) ; span_lint_and_sugg (cx , UNNECESSARY_OPTION_MAP_OR_ELSE , expr . span , msg , "consider using `unwrap_or_else`" , format ! ("{self_snippet}.unwrap_or_else({err_snippet})") , Applicability :: MachineApplicable ,) ; }
};
}
