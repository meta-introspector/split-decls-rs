// Generated macro for impl_5877 (impl)
macro_rules! Depcrate_methods_manual_is_variant_andimpl_5877 {
() => {
// Module: crate::methods::manual_is_variant_and
// Provides: {"impl_5877"}
// Dependencies: {}
impl < 'hir > TryFrom < & 'hir Expr < 'hir > > for MapFunc < 'hir > { type Error = () ; fn try_from (expr : & 'hir Expr < 'hir >) -> Result < Self , Self :: Error > { match expr . kind { ExprKind :: Closure (closure) => Ok (Self :: Closure (closure)) , ExprKind :: Path (_) => Ok (Self :: Path (expr)) , _ => Err (()) , } } }
};
}
