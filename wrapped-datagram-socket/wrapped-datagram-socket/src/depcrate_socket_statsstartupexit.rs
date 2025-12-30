// Generated macro for StartupExit (struct)
macro_rules! Depcrate_socket_statsStartupExit {
() => {
// Module: crate::socket_stats
// Provides: {"StartupExit"}
// Dependencies: {}
# [doc = " Statistics from when a CCA first exited the startup phase."] # [derive (Debug , Clone , Copy , PartialEq)] pub struct StartupExit { pub cwnd : usize , pub bandwidth : Option < u64 > , pub reason : StartupExitReason , }
};
}
