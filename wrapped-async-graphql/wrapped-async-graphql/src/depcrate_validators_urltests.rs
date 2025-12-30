// Generated macro for tests (module)
macro_rules! Depcrate_validators_urltests {
() => {
// Module: crate::validators::url
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_url () { assert ! (url (& "http" . to_string ()) . is_err ()) ; assert ! (url (& "https://google.com" . to_string ()) . is_ok ()) ; assert ! (url (& "http://localhost:80" . to_string ()) . is_ok ()) ; assert ! (url (& "ftp://localhost:80" . to_string ()) . is_ok ()) ; } }
};
}
