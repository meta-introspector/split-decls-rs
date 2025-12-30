// Generated macro for specializes (function)
macro_rules! Depcrate_specializationspecializes {
() => {
// Module: crate::specialization
// Provides: {"specializes"}
// Dependencies: {}
pub (crate) fn specializes (db : & dyn HirDatabase , specializing_impl_def_id : ImplId , parent_impl_def_id : ImplId ,) -> bool { let module = specializing_impl_def_id . loc (db) . container ; let def_map = crate_def_map (db , module . krate ()) ; if ! def_map . is_unstable_feature_enabled (& sym :: specialization) && ! def_map . is_unstable_feature_enabled (& sym :: min_specialization) { return false ; } specializes_query (db , specializing_impl_def_id , parent_impl_def_id) }
};
}
