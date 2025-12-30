// Generated macro for check (function)
macro_rules! Depcrate_ifs_ifs_same_condcheck {
() => {
// Module: crate::ifs::ifs_same_cond
// Provides: {"check"}
// Dependencies: {}
# [doc = " Implementation of `IFS_SAME_COND`."] pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , conds : & [& Expr < '_ >] , interior_mut : & mut InteriorMut < 'tcx >) { for group in search_same (conds , | e | hash_expr (cx , e) , | lhs , rhs | { if let ExprKind :: MethodCall (_ , caller , _ , _) = lhs . kind { if method_caller_is_mutable (cx , caller , interior_mut) { false } else { SpanlessEq :: new (cx) . eq_expr (lhs , rhs) } } else { eq_expr_value (cx , lhs , rhs) } } ,) { let spans : Vec < _ > = group . into_iter () . map (| expr | expr . span) . collect () ; span_lint (cx , IFS_SAME_COND , spans , "these `if` branches have the same condition") ; } }
};
}
