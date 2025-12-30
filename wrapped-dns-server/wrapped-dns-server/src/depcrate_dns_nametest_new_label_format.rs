// Generated macro for test_new_label_format (function)
macro_rules! Depcrate_dns_nametest_new_label_format {
() => {
// Module: crate::dns_name
// Provides: {"test_new_label_format"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_new_label_format () { DnsName :: new ("a") . unwrap () ; DnsName :: new ("1") . unwrap_err () ; DnsName :: new ("1a") . unwrap_err () ; DnsName :: new ("a1") . unwrap () ; DnsName :: new ("a9876543210") . unwrap () ; DnsName :: new ("-") . unwrap_err () ; DnsName :: new ("a-") . unwrap_err () ; DnsName :: new ("-a") . unwrap_err () ; DnsName :: new ("a-.b") . unwrap_err () ; DnsName :: new ("a.-b") . unwrap_err () ; DnsName :: new ("a-b") . unwrap () ; DnsName :: new ("a-0") . unwrap () ; DnsName :: new ("a---b") . unwrap () ; }
};
}
