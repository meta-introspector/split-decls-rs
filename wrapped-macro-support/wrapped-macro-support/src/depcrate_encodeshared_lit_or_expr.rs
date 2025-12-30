// Generated macro for shared_lit_or_expr (function)
macro_rules! Depcrate_encodeshared_lit_or_expr {
() => {
// Module: crate::encode
// Provides: {"shared_lit_or_expr"}
// Dependencies: {}
fn shared_lit_or_expr < 'a > (i : & 'a ast :: LitOrExpr , _intern : & 'a Interner) -> LitOrExpr < 'a > { match i { ast :: LitOrExpr :: Lit (lit) => LitOrExpr :: Lit (lit) , ast :: LitOrExpr :: Expr (expr) => LitOrExpr :: Expr (expr) , } }
};
}
