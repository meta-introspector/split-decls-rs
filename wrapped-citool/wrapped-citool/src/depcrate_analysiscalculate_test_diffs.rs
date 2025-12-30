// Generated macro for calculate_test_diffs (function)
macro_rules! Depcrate_analysiscalculate_test_diffs {
() => {
// Module: crate::analysis
// Provides: {"calculate_test_diffs"}
// Dependencies: {}
fn calculate_test_diffs (parent : TestSuiteData , current : TestSuiteData) -> HashSet < TestDiff > { let mut diffs = HashSet :: new () ; for (test , outcome) in & current . tests { match parent . tests . get (test) { Some (before) => { if before != outcome { diffs . insert (TestDiff { test : test . clone () , diff : TestOutcomeDiff :: ChangeOutcome { before : before . clone () , after : outcome . clone () , } , }) ; } } None => { diffs . insert (TestDiff { test : test . clone () , diff : TestOutcomeDiff :: Added (outcome . clone ()) , }) ; } } } for (test , outcome) in & parent . tests { if ! current . tests . contains_key (test) { diffs . insert (TestDiff { test : test . clone () , diff : TestOutcomeDiff :: Missing { before : outcome . clone () } , }) ; } } diffs }
};
}
