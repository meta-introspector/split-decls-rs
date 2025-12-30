// Generated macro for is_str_literal (function)
macro_rules! Depcrate_unnecessary_literal_boundis_str_literal {
() => {
// Module: crate::unnecessary_literal_bound
// Provides: {"is_str_literal"}
// Dependencies: {}
fn is_str_literal (expr : & Expr < '_ >) -> bool { matches ! (expr . kind , ExprKind :: Lit (Lit { node : LitKind :: Str (..) , .. }) ,) }
};
}
