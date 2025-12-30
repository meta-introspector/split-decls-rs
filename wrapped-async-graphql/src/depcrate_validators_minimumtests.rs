// Generated macro for tests (module)
macro_rules! Depcrate_validators_minimumtests {
() => {
// Module: crate::validators::minimum
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_minimum () { assert ! (minimum (& 99 , 100) . is_err ()) ; assert ! (minimum (& 100 , 100) . is_ok ()) ; assert ! (minimum (& 101 , 100) . is_ok ()) ; } }
};
}
