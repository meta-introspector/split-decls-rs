// Generated macro for check_manual_split_once (function)
macro_rules! Depcrate_methods_str_splitncheck_manual_split_once {
() => {
// Module: crate::methods::str_splitn
// Provides: {"check_manual_split_once"}
// Dependencies: {}
fn check_manual_split_once (cx : & LateContext < '_ > , method_name : Symbol , expr : & Expr < '_ > , self_arg : & Expr < '_ > , pat_arg : & Expr < '_ > , usage : & IterUsage ,) { let ctxt = expr . span . ctxt () ; let (msg , reverse) = if method_name == sym :: splitn { ("manual implementation of `split_once`" , false) } else { ("manual implementation of `rsplit_once`" , true) } ; let mut app = Applicability :: MachineApplicable ; let self_snip = snippet_with_context (cx , self_arg . span , ctxt , ".." , & mut app) . 0 ; let pat_snip = snippet_with_context (cx , pat_arg . span , ctxt , ".." , & mut app) . 0 ; let sugg = match usage . kind { IterUsageKind :: NextTuple => { if reverse { format ! ("{self_snip}.rsplit_once({pat_snip}).map(|(x, y)| (y, x))") } else { format ! ("{self_snip}.split_once({pat_snip})") } } , IterUsageKind :: Nth (1) => { let (r , field) = if reverse { ("r" , 0) } else { ("" , 1) } ; match usage . unwrap_kind { Some (UnwrapKind :: Unwrap) => { format ! ("{self_snip}.{r}split_once({pat_snip}).unwrap().{field}") } , Some (UnwrapKind :: QuestionMark) => { format ! ("{self_snip}.{r}split_once({pat_snip})?.{field}") } , None => { format ! ("{self_snip}.{r}split_once({pat_snip}).map(|x| x.{field})") } , } } , IterUsageKind :: Nth (_) => return , } ; span_lint_and_sugg (cx , MANUAL_SPLIT_ONCE , usage . span , msg , "try" , sugg , app) ; }
};
}
