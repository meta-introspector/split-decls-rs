// Generated macro for impl_9689 (impl)
macro_rules! Depcrate_stringsimpl_9689 {
() => {
// Module: crate::strings
// Provides: {"impl_9689"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for StrToString { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & Expr < '_ >) { if expr . span . from_expansion () { return ; } if let ExprKind :: MethodCall (path , self_arg , [] , _) = & expr . kind && path . ident . name == sym :: to_string && let ty = cx . typeck_results () . expr_ty (self_arg) && let ty :: Ref (_ , ty , ..) = ty . kind () && ty . is_str () { span_lint_and_then (cx , STR_TO_STRING , expr . span , "`to_string()` called on a `&str`" , | diag | { let mut applicability = Applicability :: MachineApplicable ; let snippet = snippet_with_applicability (cx , self_arg . span , ".." , & mut applicability) ; diag . span_suggestion (expr . span , "try" , format ! ("{snippet}.to_owned()") , applicability) ; } ,) ; } } }
};
}
