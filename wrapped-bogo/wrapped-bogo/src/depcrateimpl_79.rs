// Generated macro for impl_79 (impl)
macro_rules! Depcrateimpl_79 {
() => {
// Module: crate
// Provides: {"impl_79"}
// Dependencies: {}
impl rustls :: KeyLog for KeyLogMemo { fn log (& self , label : & str , _client_random : & [u8] , secret : & [u8]) { match label { "CLIENT_TRAFFIC_SECRET_0" => { self . 0 . lock () . unwrap () . client_traffic_secret = secret . to_vec () } "SERVER_TRAFFIC_SECRET_0" => { self . 0 . lock () . unwrap () . server_traffic_secret = secret . to_vec () } _ => { } } } fn will_log (& self , _label : & str) -> bool { true } }
};
}
