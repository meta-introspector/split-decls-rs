// Generated macro for Algorithm (enum)
macro_rules! Depcrate_outputAlgorithm {
() => {
// Module: crate::output
// Provides: {"Algorithm"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] pub enum Algorithm { # [doc = " Simple rules, but slower to execute"] Naive , # [doc = " Optimized variant of the rules"] DatafrogOpt , # [doc = " Fast to compute, but imprecise: there can be false-positives"] # [doc = " but no false-negatives. Tailored for quick \"early return\" situations."] LocationInsensitive , # [doc = " Compares the `Naive` and `DatafrogOpt` variants to ensure they indeed"] # [doc = " compute the same errors."] Compare , # [doc = " Combination of the fast `LocationInsensitive` pre-pass, followed by"] # [doc = " the more expensive `DatafrogOpt` variant."] Hybrid , }
};
}
