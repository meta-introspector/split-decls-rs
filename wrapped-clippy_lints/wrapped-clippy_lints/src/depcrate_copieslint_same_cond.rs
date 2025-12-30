// Generated macro for lint_same_cond (function)
macro_rules! Depcrate_copieslint_same_cond {
() => {
// Module: crate::copies
// Provides: {"lint_same_cond"}
// Dependencies: {}
# [doc = " Implementation of `IFS_SAME_COND`."] fn lint_same_cond < 'tcx > (cx : & LateContext < 'tcx > , conds : & [& Expr < '_ >] , interior_mut : & mut InteriorMut < 'tcx >) { for group in search_same (conds , | e | hash_expr (cx , e) , | lhs , rhs | { if let ExprKind :: MethodCall (_ , caller , _ , _) = lhs . kind { if method_caller_is_mutable (cx , caller , interior_mut) { false } else { SpanlessEq :: new (cx) . eq_expr (lhs , rhs) } } else { eq_expr_value (cx , lhs , rhs) } } ,) { let spans : Vec < _ > = group . into_iter () . map (| expr | expr . span) . collect () ; span_lint (cx , IFS_SAME_COND , spans , "these `if` branches have the same condition") ; } }
};
}
