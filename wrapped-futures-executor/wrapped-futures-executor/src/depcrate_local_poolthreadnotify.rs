// Generated macro for ThreadNotify (struct)
macro_rules! Depcrate_local_poolThreadNotify {
() => {
// Module: crate::local_pool
// Provides: {"ThreadNotify"}
// Dependencies: {}
pub (crate) struct ThreadNotify { # [doc = " The (single) executor thread."] thread : Thread , # [doc = " A flag to ensure a wakeup (i.e. `unpark()`) is not \"forgotten\""] # [doc = " before the next `park()`, which may otherwise happen if the code"] # [doc = " being executed as part of the future(s) being polled makes use of"] # [doc = " park / unpark calls of its own, i.e. we cannot assume that no other"] # [doc = " code uses park / unpark on the executing `thread`."] unparked : AtomicBool , }
};
}
