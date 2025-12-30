// Generated macro for SyncConnection (struct)
macro_rules! Depcrate_nonblockSyncConnection {
() => {
// Module: crate::nonblock
// Provides: {"SyncConnection"}
// Dependencies: {}
# [doc = " A connection to D-Bus, Send + Sync + async version"] pub struct SyncConnection { channel : Channel , filters : Mutex < Filters < SyncFilterCb > > , replies : Mutex < Replies < SyncRepliesCb > > , timeout_maker : Option < TimeoutMakerCb > , waker : Option < WakerCb > , all_signal_matches : AtomicBool , }
};
}
