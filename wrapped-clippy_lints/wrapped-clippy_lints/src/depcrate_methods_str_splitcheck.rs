// Generated macro for check (function)
macro_rules! Depcrate_methods_str_splitcheck {
() => {
// Module: crate::methods::str_split
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'a > (cx : & LateContext < 'a > , expr : & '_ Expr < '_ > , split_recv : & 'a Expr < '_ > , split_arg : & '_ Expr < '_ >) { if let ExprKind :: MethodCall (trim_method_name , trim_recv , [] , _) = split_recv . kind && trim_method_name . ident . name == sym :: trim && cx . typeck_results () . expr_ty_adjusted (trim_recv) . peel_refs () . is_str () && ! is_const_evaluatable (cx , trim_recv) && let ExprKind :: Lit (split_lit) = split_arg . kind && (matches ! (split_lit . node , LitKind :: Char ('\n')) || matches ! (split_lit . node , LitKind :: Str (sym :: LF | sym :: CRLF , _))) { let mut app = Applicability :: MaybeIncorrect ; span_lint_and_sugg (cx , STR_SPLIT_AT_NEWLINE , expr . span , "using `str.trim().split()` with hard-coded newlines" , "use `str.lines()` instead" , format ! ("{}.lines()" , snippet_with_context (cx , trim_recv . span , expr . span . ctxt () , ".." , & mut app) . 0) , app ,) ; } }
};
}
