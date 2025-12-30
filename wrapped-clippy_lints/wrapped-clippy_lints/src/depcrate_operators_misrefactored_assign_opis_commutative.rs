// Generated macro for is_commutative (function)
macro_rules! Depcrate_operators_misrefactored_assign_opis_commutative {
() => {
// Module: crate::operators::misrefactored_assign_op
// Provides: {"is_commutative"}
// Dependencies: {}
# [must_use] fn is_commutative (op : hir :: BinOpKind) -> bool { use rustc_hir :: BinOpKind :: { Add , And , BitAnd , BitOr , BitXor , Div , Eq , Ge , Gt , Le , Lt , Mul , Ne , Or , Rem , Shl , Shr , Sub , } ; match op { Add | Mul | And | Or | BitXor | BitAnd | BitOr | Eq | Ne => true , Sub | Div | Rem | Shl | Shr | Lt | Le | Ge | Gt => false , } }
};
}
