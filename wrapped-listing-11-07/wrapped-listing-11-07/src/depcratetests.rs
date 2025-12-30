// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn it_adds_two () { let result = add_two (2) ; assert_eq ! (result , 4) ; } }
};
}
