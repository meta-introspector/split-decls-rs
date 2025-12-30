// Generated macro for try_from_header_name_tests (module)
macro_rules! Depcrate_header_valuetry_from_header_name_tests {
() => {
// Module: crate::header::value
// Provides: {"try_from_header_name_tests"}
// Dependencies: {}
# [cfg (test)] mod try_from_header_name_tests { use super :: * ; use crate :: header :: name ; # [test] fn it_converts_using_try_from () { assert_eq ! (HeaderValue :: try_from (name :: UPGRADE) . unwrap () , HeaderValue :: from_bytes (b"upgrade") . unwrap ()) ; } }
};
}
