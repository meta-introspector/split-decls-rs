// Generated macro for tests (module)
macro_rules! Depcrate_ossl_paramtests {
() => {
// Module: crate::ossl_param
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_builder_locate_octet_string () { let mut builder = OsslParamBuilder :: new () . unwrap () ; builder . add_octet_string (CStr :: from_bytes_with_nul (b"key1\0") . unwrap () , b"value1") . unwrap () ; let params = builder . to_param () . unwrap () ; assert ! (params . locate_octet_string (CStr :: from_bytes_with_nul (b"invalid\0") . unwrap ()) . is_err ()) ; assert_eq ! (params . locate_octet_string (CStr :: from_bytes_with_nul (b"key1\0") . unwrap ()) . unwrap () , b"value1") ; } }
};
}
