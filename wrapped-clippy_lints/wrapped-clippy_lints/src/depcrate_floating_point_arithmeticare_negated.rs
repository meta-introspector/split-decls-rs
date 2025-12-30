// Generated macro for are_negated (function)
macro_rules! Depcrate_floating_point_arithmeticare_negated {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"are_negated"}
// Dependencies: {}
# [doc = " If the two expressions are negations of each other, then it returns"] # [doc = " a tuple, in which the first element is true iff expr1 is the"] # [doc = " positive expressions, and the second element is the positive"] # [doc = " one of the two expressions"] # [doc = " If the two expressions are not negations of each other, then it"] # [doc = " returns None."] fn are_negated < 'a > (cx : & LateContext < '_ > , expr1 : & 'a Expr < 'a > , expr2 : & 'a Expr < 'a >) -> Option < (bool , & 'a Expr < 'a >) > { if let ExprKind :: Unary (UnOp :: Neg , expr1_negated) = & expr1 . kind && eq_expr_value (cx , expr1_negated , expr2) { return Some ((false , expr2)) ; } if let ExprKind :: Unary (UnOp :: Neg , expr2_negated) = & expr2 . kind && eq_expr_value (cx , expr1 , expr2_negated) { return Some ((true , expr1)) ; } None }
};
}
