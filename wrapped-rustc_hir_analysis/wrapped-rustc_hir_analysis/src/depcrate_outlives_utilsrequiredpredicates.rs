// Generated macro for RequiredPredicates (type)
macro_rules! Depcrate_outlives_utilsRequiredPredicates {
() => {
// Module: crate::outlives::utils
// Provides: {"RequiredPredicates"}
// Dependencies: {}
# [doc = " Tracks the `T: 'a` or `'a: 'a` predicates that we have inferred"] # [doc = " must be added to the struct header."] pub (crate) type RequiredPredicates < 'tcx > = FxIndexMap < ty :: ArgOutlivesPredicate < 'tcx > , Span > ;
};
}
