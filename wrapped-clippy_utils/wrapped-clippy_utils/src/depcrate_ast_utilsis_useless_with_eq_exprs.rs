// Generated macro for is_useless_with_eq_exprs (function)
macro_rules! Depcrate_ast_utilsis_useless_with_eq_exprs {
() => {
// Module: crate::ast_utils
// Provides: {"is_useless_with_eq_exprs"}
// Dependencies: {}
pub fn is_useless_with_eq_exprs (kind : BinOpKind) -> bool { use BinOpKind :: * ; matches ! (kind , Sub | Div | Eq | Lt | Le | Gt | Ge | Ne | And | Or | BitXor | BitAnd | BitOr) }
};
}
