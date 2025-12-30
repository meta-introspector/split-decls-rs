// Generated macro for impl_168 (impl)
macro_rules! Depcrate_lowerimpl_168 {
() => {
// Module: crate::lower
// Provides: {"impl_168"}
// Dependencies: {}
impl < 'db > LifetimeElisionKind < 'db > { # [inline] pub (crate) fn for_const (interner : DbInterner < 'db > , const_parent : ItemContainerId ,) -> LifetimeElisionKind < 'db > { match const_parent { ItemContainerId :: ExternBlockId (_) | ItemContainerId :: ModuleId (_) => { LifetimeElisionKind :: Elided (Region :: new_static (interner)) } ItemContainerId :: ImplId (_) => { LifetimeElisionKind :: StaticIfNoLifetimeInScope { only_lint : true } } ItemContainerId :: TraitId (_) => { LifetimeElisionKind :: StaticIfNoLifetimeInScope { only_lint : false } } } } # [inline] pub (crate) fn for_fn_params (data : & FunctionSignature) -> LifetimeElisionKind < 'db > { LifetimeElisionKind :: AnonymousCreateParameter { report_in_path : data . is_async () } } # [inline] pub (crate) fn for_fn_ret (interner : DbInterner < 'db >) -> LifetimeElisionKind < 'db > { LifetimeElisionKind :: Elided (Region :: error (interner)) } }
};
}
