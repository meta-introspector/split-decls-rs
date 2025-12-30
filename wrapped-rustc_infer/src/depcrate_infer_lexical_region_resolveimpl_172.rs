// Generated macro for impl_172 (impl)
macro_rules! Depcrate_infer_lexical_region_resolveimpl_172 {
() => {
// Module: crate::infer::lexical_region_resolve
// Provides: {"impl_172"}
// Dependencies: {}
impl < 'tcx > RegionResolutionError < 'tcx > { pub fn origin (& self) -> & SubregionOrigin < 'tcx > { match self { RegionResolutionError :: ConcreteFailure (origin , _ , _) | RegionResolutionError :: GenericBoundFailure (origin , _ , _) | RegionResolutionError :: SubSupConflict (_ , _ , origin , _ , _ , _ , _) | RegionResolutionError :: UpperBoundUniverseConflict (_ , _ , _ , origin , _) | RegionResolutionError :: CannotNormalize (_ , origin) => origin , } } }
};
}
