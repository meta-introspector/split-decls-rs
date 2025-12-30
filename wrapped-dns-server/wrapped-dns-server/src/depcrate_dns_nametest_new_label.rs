// Generated macro for test_new_label (function)
macro_rules! Depcrate_dns_nametest_new_label {
() => {
// Module: crate::dns_name
// Provides: {"test_new_label"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_new_label () { assert_eq ! ("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-0123456789" , DnsName :: new ("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-0123456789") . unwrap () . inner ()) ; assert_eq ! ("aB-Cd.eFg" , DnsName :: new ("aB-Cd.eFg") . unwrap () . inner ()) ; }
};
}
