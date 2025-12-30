// Generated macro for impl_864 (impl)
macro_rules! Depcrate_opimpl_864 {
() => {
// Module: crate::op
// Provides: {"impl_864"}
// Dependencies: {}
impl From < hir :: AssignOpKind > for BinOpCategory { fn from (op : hir :: AssignOpKind) -> BinOpCategory { use hir :: AssignOpKind :: * ; match op { ShlAssign | ShrAssign => BinOpCategory :: Shift , AddAssign | SubAssign | MulAssign | DivAssign | RemAssign => BinOpCategory :: Math , BitXorAssign | BitAndAssign | BitOrAssign => BinOpCategory :: Bitwise , } } }
};
}
