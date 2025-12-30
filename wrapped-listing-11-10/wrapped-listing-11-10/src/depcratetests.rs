// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn this_test_will_pass () { let value = prints_and_returns_10 (4) ; assert_eq ! (value , 10) ; } # [test] fn this_test_will_fail () { let value = prints_and_returns_10 (8) ; assert_eq ! (value , 5) ; } }
};
}
