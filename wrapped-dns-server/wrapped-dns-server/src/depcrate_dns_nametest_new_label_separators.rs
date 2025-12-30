// Generated macro for test_new_label_separators (function)
macro_rules! Depcrate_dns_nametest_new_label_separators {
() => {
// Module: crate::dns_name
// Provides: {"test_new_label_separators"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_new_label_separators () { DnsName :: new (".") . unwrap_err () ; assert_eq ! ("a" , DnsName :: new ("a.") . unwrap () . inner ()) ; DnsName :: new ("a..") . unwrap_err () ; DnsName :: new (".a") . unwrap_err () ; DnsName :: new ("b..a") . unwrap_err () ; DnsName :: new (".b.a") . unwrap_err () ; }
};
}
