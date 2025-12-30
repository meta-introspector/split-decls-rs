// Generated macro for ClosureOutlivesRequirement (struct)
macro_rules! DepcrateClosureOutlivesRequirement {
() => {
// Module: crate
// Provides: {"ClosureOutlivesRequirement"}
// Dependencies: {}
# [doc = " Indicates an outlives-constraint between a type or between two"] # [doc = " free regions declared on the closure."] # [derive (Copy , Clone , Debug)] pub struct ClosureOutlivesRequirement < 'tcx > { pub subject : ClosureOutlivesSubject < 'tcx > , pub outlived_free_region : ty :: RegionVid , pub blame_span : Span , pub category : ConstraintCategory < 'tcx > , }
};
}
