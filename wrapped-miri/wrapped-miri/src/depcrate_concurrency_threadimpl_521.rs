// Generated macro for impl_521 (impl)
macro_rules! Depcrate_concurrency_threadimpl_521 {
() => {
// Module: crate::concurrency::thread
// Provides: {"impl_521"}
// Dependencies: {}
impl Timeout { # [doc = " How long do we have to wait from now until the specified time?"] fn get_wait_time (& self , clock : & MonotonicClock) -> Duration { match self { Timeout :: Monotonic (instant) => instant . duration_since (clock . now ()) , Timeout :: RealTime (time) => time . duration_since (SystemTime :: now ()) . unwrap_or (Duration :: ZERO) , } } # [doc = " Will try to add `duration`, but if that overflows it may add less."] fn add_lossy (& self , duration : Duration) -> Self { match self { Timeout :: Monotonic (i) => Timeout :: Monotonic (i . add_lossy (duration)) , Timeout :: RealTime (s) => { Timeout :: RealTime (s . checked_add (duration) . unwrap_or_else (| | s . checked_add (Duration :: from_secs (3600)) . unwrap ()) ,) } } } }
};
}
