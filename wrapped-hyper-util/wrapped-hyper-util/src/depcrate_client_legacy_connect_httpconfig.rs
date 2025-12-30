// Generated macro for Config (struct)
macro_rules! Depcrate_client_legacy_connect_httpConfig {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"Config"}
// Dependencies: {}
# [derive (Clone)] struct Config { connect_timeout : Option < Duration > , enforce_http : bool , happy_eyeballs_timeout : Option < Duration > , tcp_keepalive_config : TcpKeepaliveConfig , local_address_ipv4 : Option < Ipv4Addr > , local_address_ipv6 : Option < Ipv6Addr > , nodelay : bool , reuse_address : bool , send_buffer_size : Option < usize > , recv_buffer_size : Option < usize > , # [cfg (any (target_os = "android" , target_os = "fuchsia" , target_os = "linux"))] interface : Option < String > , # [cfg (any (target_os = "illumos" , target_os = "ios" , target_os = "macos" , target_os = "solaris" , target_os = "tvos" , target_os = "visionos" , target_os = "watchos" ,))] interface : Option < std :: ffi :: CString > , # [cfg (any (target_os = "android" , target_os = "fuchsia" , target_os = "linux"))] tcp_user_timeout : Option < Duration > , }
};
}
