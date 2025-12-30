// Generated macro for Baseline (enum)
macro_rules! DepcrateBaseline {
() => {
// Module: crate
// Provides: {"Baseline"}
// Dependencies: {}
# [doc = " Baseline describes how the `baseline_directory` is handled."] # [derive (Debug , Clone , Copy)] pub enum Baseline { # [doc = " `CompareLenient` compares against a previous saved version of the baseline."] # [doc = " If a previous baseline does not exist, the benchmark is run as normal but no comparison occurs."] CompareLenient , # [doc = " `CompareStrict` compares against a previous saved version of the baseline."] # [doc = " If a previous baseline does not exist, a panic occurs."] CompareStrict , # [doc = " `Save` writes the benchmark results to the baseline directory,"] # [doc = " overwriting any results that were previously there."] Save , # [doc = " `Discard` benchmark results."] Discard , }
};
}
