// Generated macro for default_verify_paths (function)
macro_rules! Depcrate_ssl_testdefault_verify_paths {
() => {
// Module: crate::ssl::test
// Provides: {"default_verify_paths"}
// Dependencies: {}
# [test] # [cfg_attr (libressl , ignore)] # [cfg_attr (target_os = "windows" , ignore)] # [cfg_attr (all (target_os = "macos" , feature = "vendored") , ignore)] fn default_verify_paths () { let mut ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () ; ctx . set_default_verify_paths () . unwrap () ; ctx . set_verify (SslVerifyMode :: PEER) ; let ctx = ctx . build () ; let s = match TcpStream :: connect ("google.com:443") { Ok (s) => s , Err (_) => return , } ; let mut ssl = Ssl :: new (& ctx) . unwrap () ; ssl . set_hostname ("google.com") . unwrap () ; let mut socket = ssl . connect (s) . unwrap () ; socket . write_all (b"GET / HTTP/1.0\r\n\r\n") . unwrap () ; let mut result = vec ! [] ; socket . read_to_end (& mut result) . unwrap () ; println ! ("{}" , String :: from_utf8_lossy (& result)) ; assert ! (result . starts_with (b"HTTP/1.0")) ; assert ! (result . ends_with (b"</HTML>\r\n") || result . ends_with (b"</html>")) ; }
};
}
