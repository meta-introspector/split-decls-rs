// Generated macro for echo_v4 (function)
macro_rules! Depcrate_testsecho_v4 {
() => {
// Module: crate::tests
// Provides: {"echo_v4"}
// Dependencies: {}
# [test] # [cfg_attr (target_os = "solaris" , ignore = "Sometimes hangs in poll() on Solaris")] fn echo_v4 () { run_echo (EchoArgs { client_addr : SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: UNSPECIFIED) , 0) , server_addr : SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: LOCALHOST) , 0) , nr_streams : 1 , stream_size : 10 * 1024 , receive_window : None , stream_receive_window : None , }) ; }
};
}
