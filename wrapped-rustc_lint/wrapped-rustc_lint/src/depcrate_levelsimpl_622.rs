// Generated macro for impl_622 (impl)
macro_rules! Depcrate_levelsimpl_622 {
() => {
// Module: crate::levels
// Provides: {"impl_622"}
// Dependencies: {}
impl < 'tcx > LintLevelsBuilder < '_ , LintLevelQueryMap < 'tcx > > { fn add_id (& mut self , hir_id : HirId) { self . provider . cur = hir_id ; self . add (self . provider . attrs . get (hir_id . local_id) , hir_id == hir :: CRATE_HIR_ID , Some (hir_id) ,) ; } }
};
}
