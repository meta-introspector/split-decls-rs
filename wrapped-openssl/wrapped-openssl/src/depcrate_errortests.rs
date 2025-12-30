// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [cfg (not (ossl310))] use crate :: nid :: Nid ; # [test] # [cfg (not (ossl310))] fn test_error_library_code () { let stack = Nid :: create ("not-an-oid" , "invalid" , "invalid") . unwrap_err () ; let errors = stack . errors () ; # [cfg (not (any (boringssl , awslc)))] assert_eq ! (errors [0] . library_code () , ffi :: ERR_LIB_ASN1) ; # [cfg (any (boringssl , awslc))] assert_eq ! (errors [0] . library_code () , ffi :: ERR_LIB_OBJ as libc :: c_int) ; } }
};
}
