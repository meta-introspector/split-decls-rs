// Generated macro for impl_966 (impl)
macro_rules! Depcrateimpl_966 {
() => {
// Module: crate
// Provides: {"impl_966"}
// Dependencies: {}
impl < 'hir > MaybePath < 'hir > for Pat < 'hir > { fn hir_id (& self) -> HirId { self . hir_id } fn qpath_opt (& self) -> Option < & QPath < 'hir > > { match & self . kind { PatKind :: Expr (PatExpr { kind : PatExprKind :: Path (qpath) , .. }) => Some (qpath) , _ => None , } } }
};
}
