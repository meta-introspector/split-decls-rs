// Generated macro for impl_349 (impl)
macro_rules! Depcrate_infer_region_constraintsimpl_349 {
() => {
// Module: crate::infer::region_constraints
// Provides: {"impl_349"}
// Dependencies: {}
impl < 'tcx > RegionConstraintStorage < 'tcx > { # [inline] pub (crate) fn with_log < 'a > (& 'a mut self , undo_log : & 'a mut InferCtxtUndoLogs < 'tcx > ,) -> RegionConstraintCollector < 'a , 'tcx > { RegionConstraintCollector { storage : self , undo_log } } }
};
}
