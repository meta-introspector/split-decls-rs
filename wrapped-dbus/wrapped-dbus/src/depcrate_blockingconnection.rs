// Generated macro for Connection (struct)
macro_rules! Depcrate_blockingConnection {
() => {
// Module: crate::blocking
// Provides: {"Connection"}
// Dependencies: {}
# [doc = " A connection to D-Bus, non-async version where callbacks are Send but not Sync."] pub struct Connection { channel : Channel , filters : RefCell < Filters < FilterCb > > , all_signal_matches : AtomicBool , }
};
}
