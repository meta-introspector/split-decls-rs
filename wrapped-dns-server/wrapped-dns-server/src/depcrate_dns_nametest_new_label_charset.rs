// Generated macro for test_new_label_charset (function)
macro_rules! Depcrate_dns_nametest_new_label_charset {
() => {
// Module: crate::dns_name
// Provides: {"test_new_label_charset"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_new_label_charset () { const ALLOWED : & str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-." ; for c in ALLOWED . chars () { let value = format ! ("a{c}a") ; DnsName :: new (& value) . expect (& value) ; } for b in 0 ..= 255_u8 { let c = char :: from (b) ; if ! ALLOWED . contains (c) { let value = format ! ("a{c}a") ; assert_eq ! (< Result < DnsName , String >>:: Err (format ! ("not a valid DNS name: {value:?}")) , DnsName :: new (& value)) ; } } assert_eq ! (< Result < DnsName , String >>:: Err ("not a valid DNS name: \"a\u{263A}\"" . to_string ()) , DnsName :: new ("a\u{263A}")) ; }
};
}
