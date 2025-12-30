// Generated macro for echo_v6 (function)
macro_rules! Depcrate_testsecho_v6 {
() => {
// Module: crate::tests
// Provides: {"echo_v6"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "solaris" , target_os = "illumos") , ignore = "Fails on Solaris and Illumos")] fn echo_v6 () { run_echo (EchoArgs { client_addr : SocketAddr :: new (IpAddr :: V6 (Ipv6Addr :: UNSPECIFIED) , 0) , server_addr : SocketAddr :: new (IpAddr :: V6 (Ipv6Addr :: LOCALHOST) , 0) , nr_streams : 1 , stream_size : 10 * 1024 , receive_window : None , stream_receive_window : None , }) ; }
};
}
