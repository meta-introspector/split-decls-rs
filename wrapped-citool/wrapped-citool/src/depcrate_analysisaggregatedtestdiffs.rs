// Generated macro for AggregatedTestDiffs (struct)
macro_rules! Depcrate_analysisAggregatedTestDiffs {
() => {
// Module: crate::analysis
// Provides: {"AggregatedTestDiffs"}
// Dependencies: {}
# [doc = " Represents a difference in the outcome of tests between a base and a current commit."] # [doc = " Maps test diffs to jobs that contained them."] # [derive (Debug)] struct AggregatedTestDiffs { diffs : HashMap < TestDiff , Vec < JobName > > , }
};
}
