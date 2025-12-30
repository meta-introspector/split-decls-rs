// Generated macro for lock_and_then (function)
macro_rules! Depcrate_io_splitlock_and_then {
() => {
// Module: crate::io::split
// Provides: {"lock_and_then"}
// Dependencies: {}
fn lock_and_then < T , U , E , F > (lock : & BiLock < T > , cx : & mut Context < '_ > , f : F) -> Poll < Result < U , E > > where F : FnOnce (Pin < & mut T > , & mut Context < '_ >) -> Poll < Result < U , E > > , { let mut l = ready ! (lock . poll_lock (cx)) ; f (l . as_pin_mut () , cx) }
};
}
