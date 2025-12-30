// Generated macro for SyncConnection (struct)
macro_rules! Depcrate_blockingSyncConnection {
() => {
// Module: crate::blocking
// Provides: {"SyncConnection"}
// Dependencies: {}
# [doc = " A connection to D-Bus, Send + Sync + non-async version"] pub struct SyncConnection { channel : Channel , filters : Mutex < Filters < SyncFilterCb > > , all_signal_matches : AtomicBool , }
};
}
