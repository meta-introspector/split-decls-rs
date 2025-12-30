// Generated macro for lint_if_same_then_else (function)
macro_rules! Depcrate_copieslint_if_same_then_else {
() => {
// Module: crate::copies
// Provides: {"lint_if_same_then_else"}
// Dependencies: {}
fn lint_if_same_then_else (cx : & LateContext < '_ > , conds : & [& Expr < '_ >] , blocks : & [& Block < '_ >]) -> bool { let mut eq = SpanlessEq :: new (cx) ; blocks . array_windows :: < 2 > () . enumerate () . fold (true , | all_eq , (i , & [lhs , rhs]) | { if eq . eq_block (lhs , rhs) && ! has_let_expr (conds [i]) && conds . get (i + 1) . is_none_or (| e | ! has_let_expr (e)) { span_lint_and_note (cx , IF_SAME_THEN_ELSE , lhs . span , "this `if` has identical blocks" , Some (rhs . span) , "same as this" ,) ; all_eq } else { false } }) }
};
}
