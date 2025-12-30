// Generated macro for echo_dualstack (function)
macro_rules! Depcrate_testsecho_dualstack {
() => {
// Module: crate::tests
// Provides: {"echo_dualstack"}
// Dependencies: {}
# [test] # [cfg_attr (target_os = "solaris" , ignore = "Hangs in poll() on Solaris")] fn echo_dualstack () { run_echo (EchoArgs { client_addr : SocketAddr :: new (IpAddr :: V6 (Ipv6Addr :: UNSPECIFIED) , 0) , server_addr : SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: LOCALHOST) , 0) , nr_streams : 1 , stream_size : 10 * 1024 , receive_window : None , stream_receive_window : None , }) ; }
};
}
