// Generated macro for spawn_local (function)
macro_rules! Depcratespawn_local {
() => {
// Module: crate
// Provides: {"spawn_local"}
// Dependencies: {}
# [doc = " Runs a Rust `Future` on the current thread."] # [doc = ""] # [doc = " The `future` must be `'static` because it will be scheduled"] # [doc = " to run in the background and cannot contain any stack references."] # [doc = ""] # [doc = " The `future` will always be run on the next microtask tick even if it"] # [doc = " immediately returns `Poll::Ready`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function has the same panic behavior as `future_to_promise`."] # [inline] pub fn spawn_local < F > (future : F) where F : Future < Output = () > + 'static , { task :: Task :: spawn (future) ; }
};
}
