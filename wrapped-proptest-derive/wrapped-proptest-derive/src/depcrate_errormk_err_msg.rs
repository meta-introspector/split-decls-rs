// Generated macro for mk_err_msg (macro)
macro_rules! Depcrate_errormk_err_msg {
() => {
// Module: crate::error
// Provides: {"mk_err_msg"}
// Dependencies: {}
# [doc = " Produce an error string with the error `$code` which corresponds"] # [doc = " to the given `$message`."] macro_rules ! mk_err_msg { ($ code : ident , $ msg : expr) => { format ! ("[proptest_derive, {}] during #[derive(Arbitrary)]:\n{} Please see: https://proptest-rs.github.io/proptest/proptest-derive/errors.html#{} for more information." , stringify ! ($ code) , $ msg , (stringify ! ($ code)) . to_lowercase ()) } ; }
};
}
