// Generated macro for detail_exit (function)
macro_rules! Depcrate_utildetail_exit {
() => {
// Module: crate::util
// Provides: {"detail_exit"}
// Dependencies: {}
# [doc = " If code is not 0 (successful exit status), exit status is 101 (rust's default error code.)"] # [doc = " If `is_test` true and code is an error code, it will cause a panic."] pub fn detail_exit (code : i32 , is_test : bool) -> ! { if is_test { panic ! ("status code: {code}") ; } else { if CiEnv :: is_ci () { let bootstrap_args = std :: env :: args () . skip (1) . map (| a | a . to_string ()) . collect :: < Vec < _ > > () . join (" ") ; eprintln ! ("Bootstrap failed while executing `{bootstrap_args}`") ; } std :: process :: exit (code) ; } }
};
}
