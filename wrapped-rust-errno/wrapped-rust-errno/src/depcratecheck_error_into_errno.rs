// Generated macro for check_error_into_errno (function)
macro_rules! Depcratecheck_error_into_errno {
() => {
// Module: crate
// Provides: {"check_error_into_errno"}
// Dependencies: {}
# [cfg (feature = "std")] # [test] fn check_error_into_errno () { const ERROR_CODE : i32 = 1 ; let error = io :: Error :: from_raw_os_error (ERROR_CODE) ; let new_error : io :: Error = Errno (ERROR_CODE) . into () ; assert_eq ! (error . kind () , new_error . kind ()) ; }
};
}
