// Generated macro for SignalType (enum)
macro_rules! Depcrate_signalSignalType {
() => {
// Module: crate::signal
// Provides: {"SignalType"}
// Dependencies: {}
# [doc = " A cross-platform way to represent Ctrl-C or program termination signal. Other"] # [doc = " signals/events are supported via `Other`-variant."] # [derive (Debug)] pub enum SignalType { # [doc = " Ctrl-C"] Ctrlc , # [doc = " Program termination"] # [doc = " Maps to `SIGTERM` and `SIGHUP` on *nix, `CTRL_CLOSE_EVENT` on Windows."] Termination , # [doc = " Other signal/event using platform-specific data"] Other (platform :: Signal) , }
};
}
