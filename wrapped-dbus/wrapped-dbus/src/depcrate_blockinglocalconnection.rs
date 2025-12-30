// Generated macro for LocalConnection (struct)
macro_rules! Depcrate_blockingLocalConnection {
() => {
// Module: crate::blocking
// Provides: {"LocalConnection"}
// Dependencies: {}
# [doc = " A connection to D-Bus, thread local + non-async version"] pub struct LocalConnection { channel : Channel , filters : RefCell < Filters < LocalFilterCb > > , all_signal_matches : AtomicBool , }
};
}
