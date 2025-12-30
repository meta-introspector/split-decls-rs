// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn test_mk_err_msg_format () { assert_eq ! (mk_err_msg ! (E0001 , "This is a sample error message.") , "[proptest_derive, E0001] during #[derive(Arbitrary)]:\nThis is a sample error message. Please see: https://proptest-rs.github.io/proptest/proptest-derive/errors.html#e0001 for more information.") ; } }
};
}
