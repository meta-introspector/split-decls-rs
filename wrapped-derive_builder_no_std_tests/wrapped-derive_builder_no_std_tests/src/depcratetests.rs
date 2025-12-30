// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_builder_ok () { assert_eq ! (build_foo_ok () . bar , 42) ; } # [test] fn test_builder_err () { assert_eq ! (build_foo_err () . as_deref () , Some ("`bar` must be initialized")) ; } }
};
}
