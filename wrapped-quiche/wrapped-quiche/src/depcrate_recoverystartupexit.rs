// Generated macro for StartupExit (struct)
macro_rules! Depcrate_recoveryStartupExit {
() => {
// Module: crate::recovery
// Provides: {"StartupExit"}
// Dependencies: {}
# [doc = " Statistics from when a CCA first exited the startup phase."] # [derive (Debug , Clone , Copy , PartialEq)] pub struct StartupExit { # [doc = " The congestion_window recorded at Startup exit."] pub cwnd : usize , # [doc = " The bandwidth estimate recorded at Startup exit."] pub bandwidth : Option < u64 > , # [doc = " The reason a CCA exited the startup phase."] pub reason : StartupExitReason , }
};
}
