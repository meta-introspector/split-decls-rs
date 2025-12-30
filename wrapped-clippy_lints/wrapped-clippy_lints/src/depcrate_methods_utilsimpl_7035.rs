// Generated macro for impl_7035 (impl)
macro_rules! Depcrate_methods_utilsimpl_7035 {
() => {
// Module: crate::methods::utils
// Provides: {"impl_7035"}
// Dependencies: {}
impl < 'tcx > CloneOrCopyVisitor < '_ , 'tcx > { fn is_binding (& self , expr : & Expr < 'tcx >) -> bool { self . binding_hir_ids . iter () . any (| & hir_id | expr . res_local_id () == Some (hir_id)) } }
};
}
