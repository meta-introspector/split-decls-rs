// Generated macro for VarianceExtractor (struct)
macro_rules! Depcrate_polonius_liveness_constraintsVarianceExtractor {
() => {
// Module: crate::polonius::liveness_constraints
// Provides: {"VarianceExtractor"}
// Dependencies: {}
# [doc = " Extracts variances for regions contained within types. Follows the same structure as"] # [doc = " `rustc_infer`'s `Generalizer`: we try to relate a type with itself to track and extract the"] # [doc = " variances of regions."] struct VarianceExtractor < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , ambient_variance : ty :: Variance , directions : & 'a mut BTreeMap < RegionVid , ConstraintDirection > , universal_regions : & 'a UniversalRegions < 'tcx > , }
};
}
