// Generated macro for impl_244 (impl)
macro_rules! Depcrate_hirimpl_244 {
() => {
// Module: crate::hir
// Provides: {"impl_244"}
// Dependencies: {}
impl < 'hir > WhereBoundPredicate < 'hir > { # [doc = " Returns `true` if `param_def_id` matches the `bounded_ty` of this predicate."] pub fn is_param_bound (& self , param_def_id : DefId) -> bool { self . bounded_ty . as_generic_param () . is_some_and (| (def_id , _) | def_id == param_def_id) } }
};
}
