// Generated macro for tests (module)
macro_rules! Depcrate_validators_multiple_oftests {
() => {
// Module: crate::validators::multiple_of
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_multiple_of () { assert ! (multiple_of (& 5 , 3) . is_err ()) ; assert ! (multiple_of (& 6 , 3) . is_ok ()) ; assert ! (multiple_of (& 0 , 3) . is_err ()) ; } }
};
}
