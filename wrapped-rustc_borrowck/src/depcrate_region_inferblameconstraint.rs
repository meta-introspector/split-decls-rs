// Generated macro for BlameConstraint (struct)
macro_rules! Depcrate_region_inferBlameConstraint {
() => {
// Module: crate::region_infer
// Provides: {"BlameConstraint"}
// Dependencies: {}
# [derive (Clone , Debug)] pub (crate) struct BlameConstraint < 'tcx > { pub category : ConstraintCategory < 'tcx > , pub from_closure : bool , pub cause : ObligationCause < 'tcx > , pub variance_info : ty :: VarianceDiagInfo < TyCtxt < 'tcx > > , }
};
}
