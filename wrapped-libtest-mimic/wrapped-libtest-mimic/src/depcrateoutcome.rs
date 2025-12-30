// Generated macro for Outcome (enum)
macro_rules! DepcrateOutcome {
() => {
// Module: crate
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The outcome of performing a test/benchmark."] # [derive (Debug , Clone)] enum Outcome { # [doc = " The test passed."] Passed , # [doc = " The test or benchmark failed."] Failed (Failed) , # [doc = " The test or benchmark was ignored."] Ignored , # [doc = " The benchmark was successfully run."] Measured (Measurement) , }
};
}
