// Generated macro for get_expr (function)
macro_rules! Depcrate_parserget_expr {
() => {
// Module: crate::parser
// Provides: {"get_expr"}
// Dependencies: {}
fn get_expr (mut expr : & syn :: Expr) -> & syn :: Expr { while let syn :: Expr :: Group (g) = expr { expr = & g . expr ; } expr }
};
}
