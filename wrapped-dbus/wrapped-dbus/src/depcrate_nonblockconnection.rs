// Generated macro for Connection (struct)
macro_rules! Depcrate_nonblockConnection {
() => {
// Module: crate::nonblock
// Provides: {"Connection"}
// Dependencies: {}
# [doc = " A connection to D-Bus, async version, which is Send but not Sync."] pub struct Connection { channel : Channel , filters : RefCell < Filters < FilterCb > > , replies : RefCell < Replies < RepliesCb > > , timeout_maker : Option < TimeoutMakerCb > , waker : Option < WakerCb > , all_signal_matches : AtomicBool , }
};
}
