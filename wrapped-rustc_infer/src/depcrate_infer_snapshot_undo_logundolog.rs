// Generated macro for UndoLog (enum)
macro_rules! Depcrate_infer_snapshot_undo_logUndoLog {
() => {
// Module: crate::infer::snapshot::undo_log
// Provides: {"UndoLog"}
// Dependencies: {}
# [doc = " Records the \"undo\" data for a single operation that affects some form of inference variable."] # [derive (Clone)] pub (crate) enum UndoLog < 'tcx > { DuplicateOpaqueType , OpaqueTypes (OpaqueTypeKey < 'tcx > , Option < OpaqueHiddenType < 'tcx > >) , TypeVariables (type_variable :: UndoLog < 'tcx >) , ConstUnificationTable (sv :: UndoLog < ut :: Delegate < ConstVidKey < 'tcx > > >) , IntUnificationTable (sv :: UndoLog < ut :: Delegate < ty :: IntVid > >) , FloatUnificationTable (sv :: UndoLog < ut :: Delegate < ty :: FloatVid > >) , RegionConstraintCollector (region_constraints :: UndoLog < 'tcx >) , RegionUnificationTable (sv :: UndoLog < ut :: Delegate < RegionVidKey < 'tcx > > >) , ProjectionCache (traits :: UndoLog < 'tcx >) , PushTypeOutlivesConstraint , PushRegionAssumption , PushHirTypeckPotentiallyRegionDependentGoal , }
};
}
