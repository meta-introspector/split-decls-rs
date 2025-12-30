// Generated macro for get_numeric_literal (function)
macro_rules! Depcrate_casts_unnecessary_castget_numeric_literal {
() => {
// Module: crate::casts::unnecessary_cast
// Provides: {"get_numeric_literal"}
// Dependencies: {}
fn get_numeric_literal < 'e > (expr : & 'e Expr < 'e >) -> Option < Lit > { match expr . kind { ExprKind :: Lit (lit) => Some (lit) , ExprKind :: Unary (UnOp :: Neg , e) => { if let ExprKind :: Lit (lit) = e . kind { Some (lit) } else { None } } , _ => None , } }
};
}
