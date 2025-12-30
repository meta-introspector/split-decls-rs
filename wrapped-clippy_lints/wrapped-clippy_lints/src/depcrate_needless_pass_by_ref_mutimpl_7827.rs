// Generated macro for impl_7827 (impl)
macro_rules! Depcrate_needless_pass_by_ref_mutimpl_7827 {
() => {
// Module: crate::needless_pass_by_ref_mut
// Provides: {"impl_7827"}
// Dependencies: {}
impl NeedlessPassByRefMut < '_ > { pub fn new (conf : & 'static Conf) -> Self { Self { avoid_breaking_exported_api : conf . avoid_breaking_exported_api , used_fn_def_ids : FxHashSet :: default () , fn_def_ids_to_maybe_unused_mut : FxIndexMap :: default () , } } }
};
}
