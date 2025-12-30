// Generated macro for check (function)
macro_rules! Depcrate_methods_unnecessary_sort_bycheck {
() => {
// Module: crate::methods::unnecessary_sort_by
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , recv : & 'tcx Expr < '_ > , arg : & 'tcx Expr < '_ > , is_unstable : bool ,) { match detect_lint (cx , expr , recv , arg) { Some (LintTrigger :: SortByKey (trigger)) => { let method = if is_unstable { "sort_unstable_by_key" } else { "sort_by_key" } ; span_lint_and_sugg (cx , UNNECESSARY_SORT_BY , expr . span , format ! ("consider using `{method}`") , "try" , format ! ("{}.{}(|{}| {})" , trigger . vec_name , method , trigger . closure_arg , if let Some (std_or_core) = std_or_core (cx) && trigger . reverse { format ! ("{}::cmp::Reverse({})" , std_or_core , trigger . closure_body) } else { trigger . closure_body } ,) , if trigger . reverse { Applicability :: MaybeIncorrect } else { Applicability :: MachineApplicable } ,) ; } , Some (LintTrigger :: Sort (trigger)) => { let method = if is_unstable { "sort_unstable" } else { "sort" } ; span_lint_and_sugg (cx , UNNECESSARY_SORT_BY , expr . span , format ! ("consider using `{method}`") , "try" , format ! ("{}.{}()" , trigger . vec_name , method) , Applicability :: MachineApplicable ,) ; } , None => { } , } }
};
}
