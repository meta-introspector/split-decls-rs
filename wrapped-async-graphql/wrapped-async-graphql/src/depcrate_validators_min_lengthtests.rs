// Generated macro for tests (module)
macro_rules! Depcrate_validators_min_lengthtests {
() => {
// Module: crate::validators::min_length
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_min_length () { assert ! (min_length (& "ab" . to_string () , 3) . is_err ()) ; assert ! (min_length (& "abc" . to_string () , 3) . is_ok ()) ; assert ! (min_length (& "abcd" . to_string () , 3) . is_ok ()) ; } }
};
}
