// Generated macro for lint_same_fns_in_if_cond (function)
macro_rules! Depcrate_copieslint_same_fns_in_if_cond {
() => {
// Module: crate::copies
// Provides: {"lint_same_fns_in_if_cond"}
// Dependencies: {}
# [doc = " Implementation of `SAME_FUNCTIONS_IN_IF_CONDITION`."] fn lint_same_fns_in_if_cond (cx : & LateContext < '_ > , conds : & [& Expr < '_ >]) { let eq : & dyn Fn (& & Expr < '_ > , & & Expr < '_ >) -> bool = & | & lhs , & rhs | -> bool { if lhs . span . from_expansion () || rhs . span . from_expansion () { return false ; } if eq_expr_value (cx , lhs , rhs) { return false ; } SpanlessEq :: new (cx) . eq_expr (lhs , rhs) } ; for group in search_same (conds , | e | hash_expr (cx , e) , eq) { let spans : Vec < _ > = group . into_iter () . map (| expr | expr . span) . collect () ; span_lint (cx , SAME_FUNCTIONS_IN_IF_CONDITION , spans , "these `if` branches have the same function call" ,) ; } }
};
}
