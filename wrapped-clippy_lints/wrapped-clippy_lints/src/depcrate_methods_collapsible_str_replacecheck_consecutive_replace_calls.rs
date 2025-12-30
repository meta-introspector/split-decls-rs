// Generated macro for check_consecutive_replace_calls (function)
macro_rules! Depcrate_methods_collapsible_str_replacecheck_consecutive_replace_calls {
() => {
// Module: crate::methods::collapsible_str_replace
// Provides: {"check_consecutive_replace_calls"}
// Dependencies: {}
# [doc = " Check a chain of `str::replace` calls for `collapsible_str_replace` lint."] fn check_consecutive_replace_calls < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > , replace_methods : & ReplaceMethods < 'tcx > , to_arg : & 'tcx hir :: Expr < 'tcx > ,) { let from_args = & replace_methods . from_args ; let from_arg_reprs : Vec < String > = from_args . iter () . map (| from_arg | snippet (cx , from_arg . span , "..") . to_string ()) . collect () ; let app = Applicability :: MachineApplicable ; let earliest_replace_call = replace_methods . methods . front () . unwrap () ; if let Some ((_ , _ , [..] , span_lo , _)) = method_call (earliest_replace_call) { span_lint_and_sugg (cx , COLLAPSIBLE_STR_REPLACE , expr . span . with_lo (span_lo . lo ()) , "used consecutive `str::replace` call" , "replace with" , format ! ("replace([{}], {})" , from_arg_reprs . join (", ") , snippet (cx , to_arg . span , "..") ,) , app ,) ; } }
};
}
