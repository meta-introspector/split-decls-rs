// Generated macro for RegionErrors (struct)
macro_rules! Depcrate_diagnostics_region_errorsRegionErrors {
() => {
// Module: crate::diagnostics::region_errors
// Provides: {"RegionErrors"}
// Dependencies: {}
# [doc = " A collection of errors encountered during region inference. This is needed to efficiently"] # [doc = " report errors after borrow checking."] # [doc = ""] # [doc = " Usually we expect this to either be empty or contain a small number of items, so we can avoid"] # [doc = " allocation most of the time."] pub (crate) struct RegionErrors < 'tcx > (Vec < (RegionErrorKind < 'tcx > , ErrorGuaranteed) > , TyCtxt < 'tcx >) ;
};
}
