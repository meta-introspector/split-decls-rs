// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn add_two_and_two () { let result = add_two (2) ; assert_eq ! (result , 4) ; } # [test] fn add_three_and_two () { let result = add_two (3) ; assert_eq ! (result , 5) ; } # [test] fn one_hundred () { let result = add_two (100) ; assert_eq ! (result , 102) ; } }
};
}
