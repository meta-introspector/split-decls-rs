// Generated macro for ErrorConstraintInfo (struct)
macro_rules! Depcrate_diagnostics_region_errorsErrorConstraintInfo {
() => {
// Module: crate::diagnostics::region_errors
// Provides: {"ErrorConstraintInfo"}
// Dependencies: {}
# [doc = " Information about the various region constraints involved in a borrow checker error."] # [derive (Clone , Debug)] pub (crate) struct ErrorConstraintInfo < 'tcx > { pub (super) fr : RegionVid , pub (super) outlived_fr : RegionVid , pub (super) category : ConstraintCategory < 'tcx > , pub (super) span : Span , }
};
}
