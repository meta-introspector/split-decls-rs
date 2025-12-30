// Generated macro for test_display (function)
macro_rules! Depcrate_dns_nametest_display {
() => {
// Module: crate::dns_name
// Provides: {"test_display"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_display () { assert_eq ! ("example.com" , format ! ("{}" , DnsName :: new ("example.com") . unwrap ())) ; }
};
}
