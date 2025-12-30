// Generated macro for NeedlessPassByRefMut (struct)
macro_rules! Depcrate_needless_pass_by_ref_mutNeedlessPassByRefMut {
() => {
// Module: crate::needless_pass_by_ref_mut
// Provides: {"NeedlessPassByRefMut"}
// Dependencies: {}
pub struct NeedlessPassByRefMut < 'tcx > { avoid_breaking_exported_api : bool , used_fn_def_ids : FxHashSet < LocalDefId > , fn_def_ids_to_maybe_unused_mut : FxIndexMap < LocalDefId , Vec < rustc_hir :: Ty < 'tcx > > > , }
};
}
