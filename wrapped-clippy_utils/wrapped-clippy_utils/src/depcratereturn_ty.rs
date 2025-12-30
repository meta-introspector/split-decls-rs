// Generated macro for return_ty (function)
macro_rules! Depcratereturn_ty {
() => {
// Module: crate
// Provides: {"return_ty"}
// Dependencies: {}
# [doc = " Convenience function to get the return type of a function."] pub fn return_ty < 'tcx > (cx : & LateContext < 'tcx > , fn_def_id : OwnerId) -> Ty < 'tcx > { let ret_ty = cx . tcx . fn_sig (fn_def_id) . instantiate_identity () . output () ; cx . tcx . instantiate_bound_regions_with_erased (ret_ty) }
};
}
