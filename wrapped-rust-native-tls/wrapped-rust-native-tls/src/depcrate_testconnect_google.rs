// Generated macro for connect_google (function)
macro_rules! Depcrate_testconnect_google {
() => {
// Module: crate::test
// Provides: {"connect_google"}
// Dependencies: {}
# [test] fn connect_google () { let builder = p ! (TlsConnector :: new ()) ; let s = p ! (TcpStream :: connect ("google.com:443")) ; let mut socket = p ! (builder . connect ("google.com" , s)) ; p ! (socket . write_all (b"GET / HTTP/1.0\r\n\r\n")) ; let mut result = vec ! [] ; p ! (socket . read_to_end (& mut result)) ; println ! ("{}" , String :: from_utf8_lossy (& result)) ; assert ! (result . starts_with (b"HTTP/1.0")) ; assert ! (result . ends_with (b"</HTML>\r\n") || result . ends_with (b"</html>")) ; }
};
}
