// Generated macro for Timeout (enum)
macro_rules! Depcrate_concurrency_threadTimeout {
() => {
// Module: crate::concurrency::thread
// Provides: {"Timeout"}
// Dependencies: {}
# [doc = " The moment in time when a blocked thread should be woken up."] # [derive (Debug)] enum Timeout { Monotonic (Instant) , RealTime (SystemTime) , }
};
}
