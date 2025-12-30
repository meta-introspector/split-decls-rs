// Generated macro for test_str (function)
macro_rules! Depcrate_dns_stringtest_str {
() => {
// Module: crate::dns_string
// Provides: {"test_str"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_str () { DnsString :: new ("0") . unwrap () ; DnsString :: new ("a") . unwrap () ; DnsString :: new ("a" . repeat (255) . as_str ()) . unwrap () ; assert_eq ! ("longer than 255 bytes, not a valid DNS string: 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'" , DnsString :: new ("a" . repeat (256) . as_str ()) . unwrap_err () . as_str ()) ; }
};
}
