// Generated macro for impl_880 (impl)
macro_rules! Depcrate_opaque_typesimpl_880 {
() => {
// Module: crate::opaque_types
// Provides: {"impl_880"}
// Dependencies: {}
impl < 'tcx > UsageKind < 'tcx > { fn merge (& mut self , other : UsageKind < 'tcx >) { match (& * self , & other) { (UsageKind :: HasDefiningUse , _) | (_ , UsageKind :: None) => unreachable ! () , (UsageKind :: None , _) => * self = other , (UsageKind :: NonDefiningUse (..) | UsageKind :: UnconstrainedHiddenType (..) , UsageKind :: NonDefiningUse (..) ,) => { } (UsageKind :: NonDefiningUse (..) | UsageKind :: UnconstrainedHiddenType (..) , UsageKind :: UnconstrainedHiddenType (..) | UsageKind :: HasDefiningUse ,) => * self = other , } } }
};
}
