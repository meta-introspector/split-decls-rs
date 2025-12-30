// Generated macro for tests (module)
macro_rules! Depcrate_validators_min_password_strengthtests {
() => {
// Module: crate::validators::min_password_strength
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_min_password_strength () { assert ! (min_password_strength (& "password" . to_string () , 3) . is_err ()) ; assert ! (min_password_strength (& "query" . to_string () , 3) . is_err ()) ; assert ! (min_password_strength (& "P@ssword1" . to_string () , 3) . is_err ()) ; assert ! (min_password_strength (& "" . to_string () , 3) . is_err ()) ; assert ! (min_password_strength (& "Some!Secure!Password" . to_string () , 3) . is_ok ()) ; } }
};
}
