// Generated macro for UsageKind (enum)
macro_rules! Depcrate_opaque_typesUsageKind {
() => {
// Module: crate::opaque_types
// Provides: {"UsageKind"}
// Dependencies: {}
enum UsageKind < 'tcx > { None , NonDefiningUse (OpaqueTypeKey < 'tcx > , OpaqueHiddenType < 'tcx >) , UnconstrainedHiddenType (OpaqueHiddenType < 'tcx >) , HasDefiningUse , }
};
}
