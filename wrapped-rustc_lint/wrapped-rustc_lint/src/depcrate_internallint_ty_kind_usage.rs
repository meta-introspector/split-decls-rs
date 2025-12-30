// Generated macro for lint_ty_kind_usage (function)
macro_rules! Depcrate_internallint_ty_kind_usage {
() => {
// Module: crate::internal
// Provides: {"lint_ty_kind_usage"}
// Dependencies: {}
fn lint_ty_kind_usage (cx : & LateContext < '_ > , res : & Res) -> bool { if let Some (did) = res . opt_def_id () { cx . tcx . is_diagnostic_item (sym :: TyKind , did) || cx . tcx . is_diagnostic_item (sym :: IrTyKind , did) } else { false } }
};
}
