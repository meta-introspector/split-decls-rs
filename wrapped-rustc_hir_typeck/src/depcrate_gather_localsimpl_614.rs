// Generated macro for impl_614 (impl)
macro_rules! Depcrate_gather_localsimpl_614 {
() => {
// Module: crate::gather_locals
// Provides: {"impl_614"}
// Dependencies: {}
impl < 'a > From < (& 'a hir :: LetExpr < 'a > , HirId) > for Declaration < 'a > { fn from ((let_expr , hir_id) : (& 'a hir :: LetExpr < 'a > , HirId)) -> Self { let hir :: LetExpr { pat , ty , span , init , recovered : _ } = * let_expr ; Declaration { hir_id , pat , ty , span , init : Some (init) , origin : DeclOrigin :: LetExpr } } }
};
}
