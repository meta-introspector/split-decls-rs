// Generated macro for swap_binop (function)
macro_rules! Depcrate_hir_utilsswap_binop {
() => {
// Module: crate::hir_utils
// Provides: {"swap_binop"}
// Dependencies: {}
fn swap_binop < 'a > (binop : BinOpKind , lhs : & 'a Expr < 'a > , rhs : & 'a Expr < 'a > ,) -> Option < (BinOpKind , & 'a Expr < 'a > , & 'a Expr < 'a >) > { match binop { BinOpKind :: Add | BinOpKind :: Eq | BinOpKind :: Ne | BinOpKind :: BitAnd | BinOpKind :: BitXor | BinOpKind :: BitOr => { Some ((binop , rhs , lhs)) } , BinOpKind :: Lt => Some ((BinOpKind :: Gt , rhs , lhs)) , BinOpKind :: Le => Some ((BinOpKind :: Ge , rhs , lhs)) , BinOpKind :: Ge => Some ((BinOpKind :: Le , rhs , lhs)) , BinOpKind :: Gt => Some ((BinOpKind :: Lt , rhs , lhs)) , BinOpKind :: Mul | BinOpKind :: Shl | BinOpKind :: Shr | BinOpKind :: Rem | BinOpKind :: Sub | BinOpKind :: Div | BinOpKind :: And | BinOpKind :: Or => None , } }
};
}
