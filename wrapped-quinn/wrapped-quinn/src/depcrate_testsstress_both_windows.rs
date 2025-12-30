// Generated macro for stress_both_windows (function)
macro_rules! Depcrate_testsstress_both_windows {
() => {
// Module: crate::tests
// Provides: {"stress_both_windows"}
// Dependencies: {}
# [test] # [ignore] # [cfg_attr (target_os = "solaris" , ignore = "Hangs in poll() on Solaris")] fn stress_both_windows () { run_echo (EchoArgs { client_addr : SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: UNSPECIFIED) , 0) , server_addr : SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: LOCALHOST) , 0) , nr_streams : 50 , stream_size : 25 * 1024 + 11 , receive_window : Some (37) , stream_receive_window : Some (37) , }) ; }
};
}
