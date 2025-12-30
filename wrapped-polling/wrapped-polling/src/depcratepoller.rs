// Generated macro for Poller (struct)
macro_rules! DepcratePoller {
() => {
// Module: crate
// Provides: {"Poller"}
// Dependencies: {}
# [doc = " Waits for I/O events."] pub struct Poller { poller : sys :: Poller , lock : Mutex < () > , notified : AtomicBool , }
};
}
