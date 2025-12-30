// Generated macro for impl_6885 (impl)
macro_rules! Depcrate_methods_utilsimpl_6885 {
() => {
// Module: crate::methods::utils
// Provides: {"impl_6885"}
// Dependencies: {}
impl < 'tcx > CloneOrCopyVisitor < '_ , 'tcx > { fn is_binding (& self , expr : & Expr < 'tcx >) -> bool { self . binding_hir_ids . iter () . any (| hir_id | path_to_local_id (expr , * hir_id)) } }
};
}
