// Generated macro for ty_from_hir_ty (function)
macro_rules! Depcrate_tyty_from_hir_ty {
() => {
// Module: crate::ty
// Provides: {"ty_from_hir_ty"}
// Dependencies: {}
# [doc = " Lower a [`hir::Ty`] to a [`rustc_middle::ty::Ty`]."] pub fn ty_from_hir_ty < 'tcx > (cx : & LateContext < 'tcx > , hir_ty : & hir :: Ty < 'tcx >) -> Ty < 'tcx > { cx . maybe_typeck_results () . filter (| results | results . hir_owner == hir_ty . hir_id . owner) . and_then (| results | results . node_type_opt (hir_ty . hir_id)) . unwrap_or_else (| | lower_ty (cx . tcx , hir_ty)) }
};
}
