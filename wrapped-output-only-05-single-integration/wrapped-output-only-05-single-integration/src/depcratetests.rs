// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn internal () { let result = internal_adder (2 , 2) ; assert_eq ! (result , 4) ; } }
};
}
