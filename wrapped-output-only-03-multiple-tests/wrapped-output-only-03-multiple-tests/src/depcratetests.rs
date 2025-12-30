// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn add_two_and_two () { assert_eq ! (4 , add_two (2)) ; } # [test] fn add_three_and_two () { assert_eq ! (5 , add_two (3)) ; } # [test] fn one_hundred () { assert_eq ! (102 , add_two (100)) ; } }
};
}
