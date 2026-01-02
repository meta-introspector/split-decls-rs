// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_passes/src/lib_features.rs
// Error: expected square brackets
// Problematic line: line 18


use crate::errors::{FeaturePreviouslyDeclared, FeatureStableTwice};

struct LibFeatureCollector<'tcx> {
    tcx: TyCtxt<'tcx>,
    lib_features: LibFeatures,
}
