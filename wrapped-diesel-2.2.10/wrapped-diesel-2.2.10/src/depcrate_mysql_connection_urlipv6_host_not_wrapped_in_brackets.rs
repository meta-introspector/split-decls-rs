// Generated macro for ipv6_host_not_wrapped_in_brackets (function)
macro_rules! Depcrate_mysql_connection_urlipv6_host_not_wrapped_in_brackets {
() => {
// Module: crate::mysql::connection::url
// Provides: {"ipv6_host_not_wrapped_in_brackets"}
// Dependencies: {}
# [test] fn ipv6_host_not_wrapped_in_brackets () { let host1 = CString :: new ("::1") . unwrap () ; let host2 = CString :: new ("2001:db8:85a3::8a2e:370:7334") . unwrap () ; assert_eq ! (Some (&* host1) , ConnectionOptions :: parse ("mysql://[::1]") . unwrap () . host ()) ; assert_eq ! (Some (&* host2) , ConnectionOptions :: parse ("mysql://[2001:db8:85a3::8a2e:370:7334]") . unwrap () . host ()) ; }
};
}
