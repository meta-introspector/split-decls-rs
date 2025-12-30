// Generated macro for tests (module)
macro_rules! Depcrate_validators_regextests {
() => {
// Module: crate::validators::regex
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_url () { assert ! (regex (& "123" . to_string () , "^[0-9]+$") . is_ok ()) ; assert ! (regex (& "12a3" . to_string () , "^[0-9]+$") . is_err ()) ; } }
};
}
