// Generated macro for impl_699 (impl)
macro_rules! Depcrate_mirimpl_699 {
() => {
// Module: crate::mir
// Provides: {"impl_699"}
// Dependencies: {}
impl From < hir_def :: hir :: CmpOp > for BinOp { fn from (value : hir_def :: hir :: CmpOp) -> Self { match value { hir_def :: hir :: CmpOp :: Eq { negated : false } => BinOp :: Eq , hir_def :: hir :: CmpOp :: Eq { negated : true } => BinOp :: Ne , hir_def :: hir :: CmpOp :: Ord { ordering : Ordering :: Greater , strict : false } => BinOp :: Ge , hir_def :: hir :: CmpOp :: Ord { ordering : Ordering :: Greater , strict : true } => BinOp :: Gt , hir_def :: hir :: CmpOp :: Ord { ordering : Ordering :: Less , strict : false } => BinOp :: Le , hir_def :: hir :: CmpOp :: Ord { ordering : Ordering :: Less , strict : true } => BinOp :: Lt , } } }
};
}
