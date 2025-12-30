// Generated macro for test_display (function)
macro_rules! Depcrate_dns_stringtest_display {
() => {
// Module: crate::dns_string
// Provides: {"test_display"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_display () { assert_eq ! ("\\x00abc" , format ! ("{}" , DnsString :: new (b"\x00abc") . unwrap ())) ; }
};
}
