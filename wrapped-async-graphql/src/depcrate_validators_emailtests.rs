// Generated macro for tests (module)
macro_rules! Depcrate_validators_emailtests {
() => {
// Module: crate::validators::email
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_email () { assert ! (email (& "joe@example.com" . to_string ()) . is_ok ()) ; assert ! (email (& "joe.test@example.com" . to_string ()) . is_ok ()) ; assert ! (email (& "email@example-one.com" . to_string ()) . is_ok ()) ; assert ! (email (& "1234567890@example.com" . to_string ()) . is_ok ()) ; assert ! (email (& "plainaddress" . to_string ()) . is_err ()) ; assert ! (email (& "@example.com" . to_string ()) . is_err ()) ; assert ! (email (& "email.example.com" . to_string ()) . is_err ()) ; } }
};
}
