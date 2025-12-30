// Generated macro for test_err (function)
macro_rules! Depcrate_dns_nametest_err {
() => {
// Module: crate::dns_name
// Provides: {"test_err"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_err () { assert_eq ! (< Result < DnsName , String >>:: Err ("not a valid DNS name: \"abc!\"" . to_string ()) , DnsName :: new ("abc!")) ; }
};
}
