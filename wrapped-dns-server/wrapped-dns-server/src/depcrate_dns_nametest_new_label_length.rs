// Generated macro for test_new_label_length (function)
macro_rules! Depcrate_dns_nametest_new_label_length {
() => {
// Module: crate::dns_name
// Provides: {"test_new_label_length"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_new_label_length () { DnsName :: new ("") . unwrap_err () ; DnsName :: new ("a") . unwrap () ; DnsName :: new ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa") . unwrap () ; DnsName :: new ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa") . unwrap_err () ; }
};
}
