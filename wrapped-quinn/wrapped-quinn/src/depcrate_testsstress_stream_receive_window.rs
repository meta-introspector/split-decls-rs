// Generated macro for stress_stream_receive_window (function)
macro_rules! Depcrate_testsstress_stream_receive_window {
() => {
// Module: crate::tests
// Provides: {"stress_stream_receive_window"}
// Dependencies: {}
# [test] # [ignore] # [cfg_attr (target_os = "solaris" , ignore = "Hangs in poll() on Solaris")] fn stress_stream_receive_window () { run_echo (EchoArgs { client_addr : SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: UNSPECIFIED) , 0) , server_addr : SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: LOCALHOST) , 0) , nr_streams : 2 , stream_size : 250 * 1024 + 11 , receive_window : Some (100 * 1024 * 1024) , stream_receive_window : Some (37) , }) ; }
};
}
