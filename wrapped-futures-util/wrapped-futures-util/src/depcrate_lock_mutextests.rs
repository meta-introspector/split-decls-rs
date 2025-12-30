// Generated macro for tests (module)
macro_rules! Depcrate_lock_mutextests {
() => {
// Module: crate::lock::mutex
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use std :: format ; # [test] fn test_mutex_guard_debug_not_recurse () { let mutex = Mutex :: new (42) ; let guard = mutex . try_lock () . unwrap () ; let _ = format ! ("{guard:?}") ; let guard = MutexGuard :: map (guard , | n | n) ; let _ = format ! ("{guard:?}") ; } }
};
}
