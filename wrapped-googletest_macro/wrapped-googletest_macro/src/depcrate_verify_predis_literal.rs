// Generated macro for is_literal (function)
macro_rules! Depcrate_verify_predis_literal {
() => {
// Module: crate::verify_pred
// Provides: {"is_literal"}
// Dependencies: {}
fn is_literal (expr : & Expr) -> bool { match expr { Expr :: Lit (_) => true , Expr :: Unary (unary) => matches ! (&* unary . expr , Expr :: Lit (_)) , _ => false , } }
};
}
