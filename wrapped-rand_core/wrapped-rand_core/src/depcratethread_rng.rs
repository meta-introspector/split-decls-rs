// Generated macro for thread_rng (function)
macro_rules! Depcratethread_rng {
() => {
// Module: crate
// Provides: {"thread_rng"}
// Dependencies: {}
# [doc = " Access the thread-local generator"] # [doc = ""] # [doc = " Use [`rand::rng()`](rng()) instead."] # [cfg (feature = "thread_rng")] # [deprecated (since = "0.9.0" , note = "Renamed to `rng`")] # [inline] pub fn thread_rng () -> crate :: rngs :: ThreadRng { rng () }
};
}
