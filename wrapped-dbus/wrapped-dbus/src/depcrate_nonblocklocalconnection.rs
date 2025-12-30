// Generated macro for LocalConnection (struct)
macro_rules! Depcrate_nonblockLocalConnection {
() => {
// Module: crate::nonblock
// Provides: {"LocalConnection"}
// Dependencies: {}
# [doc = " A connection to D-Bus, thread local + async version"] pub struct LocalConnection { channel : Channel , filters : RefCell < Filters < LocalFilterCb > > , replies : RefCell < Replies < LocalRepliesCb > > , timeout_maker : Option < TimeoutMakerCb > , waker : Option < WakerCb > , all_signal_matches : AtomicBool , }
};
}
