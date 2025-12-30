// Generated macro for impl_246 (impl)
macro_rules! Depcrate_hirimpl_246 {
() => {
// Module: crate::hir
// Provides: {"impl_246"}
// Dependencies: {}
impl < 'hir > WhereRegionPredicate < 'hir > { # [doc = " Returns `true` if `param_def_id` matches the `lifetime` of this predicate."] fn is_param_bound (& self , param_def_id : LocalDefId) -> bool { self . lifetime . kind == LifetimeKind :: Param (param_def_id) } }
};
}
