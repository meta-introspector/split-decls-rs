// Generated macro for lint_needless (function)
macro_rules! Depcrate_methods_str_splitnlint_needless {
() => {
// Module: crate::methods::str_splitn
// Provides: {"lint_needless"}
// Dependencies: {}
fn lint_needless (cx : & LateContext < '_ > , method_name : Symbol , expr : & Expr < '_ > , self_arg : & Expr < '_ > , pat_arg : & Expr < '_ >) { let mut app = Applicability :: MachineApplicable ; let r = if method_name == sym :: splitn { "" } else { "r" } ; span_lint_and_sugg (cx , NEEDLESS_SPLITN , expr . span , format ! ("unnecessary use of `{r}splitn`") , "try" , format ! ("{}.{r}split({})" , snippet_with_context (cx , self_arg . span , expr . span . ctxt () , ".." , & mut app) . 0 , snippet_with_context (cx , pat_arg . span , expr . span . ctxt () , ".." , & mut app) . 0 ,) , app ,) ; }
};
}
