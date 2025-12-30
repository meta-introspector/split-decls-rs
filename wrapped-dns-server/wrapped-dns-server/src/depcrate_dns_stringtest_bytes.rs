// Generated macro for test_bytes (function)
macro_rules! Depcrate_dns_stringtest_bytes {
() => {
// Module: crate::dns_string
// Provides: {"test_bytes"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_bytes () { DnsString :: new ([]) . unwrap () ; DnsString :: new ([97u8]) . unwrap () ; DnsString :: new ([97u8 ; 255]) . unwrap () ; assert_eq ! ("longer than 255 bytes, not a valid DNS string: 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'" , DnsString :: new ([97u8 ; 256]) . unwrap_err () . as_str ()) ; for c in 0 ..= 255u8 { let array = [c] ; DnsString :: new (array) . unwrap_or_else (| _ | panic ! ("{}" , escape_ascii (& array))) ; } }
};
}
