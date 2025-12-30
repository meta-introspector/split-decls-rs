// Generated macro for CompareOutcome (enum)
macro_rules! Depcrate_runtestCompareOutcome {
() => {
// Module: crate::runtest
// Provides: {"CompareOutcome"}
// Dependencies: {}
# [doc = " Outcome of comparing a stream to a blessed file,"] # [doc = " e.g. `.stderr` and `.fixed`."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] enum CompareOutcome { # [doc = " Expected and actual outputs are the same"] Same , # [doc = " Outputs differed but were blessed"] Blessed , # [doc = " Outputs differed and an error should be emitted"] Differed , }
};
}
