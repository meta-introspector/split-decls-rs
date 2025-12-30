// Generated macro for signal_handler (module)
macro_rules! Depcratesignal_handler {
() => {
// Module: crate
// Provides: {"signal_handler"}
// Dependencies: {}
# [cfg (not (all (not (miri) , unix , any (target_env = "gnu" , target_os = "macos"))))] mod signal_handler { # [doc = " On platforms which don't support our signal handler's requirements,"] # [doc = " simply use the default signal handler provided by std."] pub (super) fn install () { } }
};
}
