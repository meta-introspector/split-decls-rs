// Generated macro for subtracts_one (function)
macro_rules! Depcrate_implicit_saturating_subsubtracts_one {
() => {
// Module: crate::implicit_saturating_sub
// Provides: {"subtracts_one"}
// Dependencies: {}
fn subtracts_one < 'a > (cx : & LateContext < '_ > , expr : & 'a Expr < 'a >) -> Option < & 'a Expr < 'a > > { match peel_blocks_with_stmt (expr) . kind { ExprKind :: AssignOp (ref op1 , target , value) => { (AssignOpKind :: SubAssign == op1 . node && is_integer_literal (value , 1)) . then_some (target) } , ExprKind :: Assign (target , value , _) => { if let ExprKind :: Binary (ref op1 , left1 , right1) = value . kind && BinOpKind :: Sub == op1 . node && SpanlessEq :: new (cx) . eq_expr (left1 , target) && is_integer_literal (right1 , 1) { Some (target) } else { None } } , _ => None , } }
};
}
