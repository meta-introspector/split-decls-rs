// Generated macro for tests (module)
macro_rules! Depcrate_validators_maximumtests {
() => {
// Module: crate::validators::maximum
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_maximum () { assert ! (maximum (& 99 , 100) . is_ok ()) ; assert ! (maximum (& 100 , 100) . is_ok ()) ; assert ! (maximum (& 101 , 100) . is_err ()) ; } }
};
}
