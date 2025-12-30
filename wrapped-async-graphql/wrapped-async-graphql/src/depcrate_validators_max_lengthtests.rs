// Generated macro for tests (module)
macro_rules! Depcrate_validators_max_lengthtests {
() => {
// Module: crate::validators::max_length
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_max_length () { assert ! (max_length (& "ab" . to_string () , 3) . is_ok ()) ; assert ! (max_length (& "abc" . to_string () , 3) . is_ok ()) ; assert ! (max_length (& "abcd" . to_string () , 3) . is_err ()) ; } }
};
}
