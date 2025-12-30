// Generated macro for self_ty (function)
macro_rules! Depcrate_ty_type_certaintyself_ty {
() => {
// Module: crate::ty::type_certainty
// Provides: {"self_ty"}
// Dependencies: {}
fn self_ty < 'tcx > (cx : & LateContext < 'tcx > , method_def_id : DefId) -> Ty < 'tcx > { cx . tcx . fn_sig (method_def_id) . skip_binder () . inputs () . skip_binder () [0] }
};
}
