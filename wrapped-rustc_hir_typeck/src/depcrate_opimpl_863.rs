// Generated macro for impl_863 (impl)
macro_rules! Depcrate_opimpl_863 {
() => {
// Module: crate::op
// Provides: {"impl_863"}
// Dependencies: {}
impl From < hir :: BinOpKind > for BinOpCategory { fn from (op : hir :: BinOpKind) -> BinOpCategory { use hir :: BinOpKind :: * ; match op { Shl | Shr => BinOpCategory :: Shift , Add | Sub | Mul | Div | Rem => BinOpCategory :: Math , BitXor | BitAnd | BitOr => BinOpCategory :: Bitwise , Eq | Ne | Lt | Le | Ge | Gt => BinOpCategory :: Comparison , And | Or => BinOpCategory :: Shortcircuit , } } }
};
}
