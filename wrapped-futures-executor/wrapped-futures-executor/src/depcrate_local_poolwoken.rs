// Generated macro for woken (function)
macro_rules! Depcrate_local_poolwoken {
() => {
// Module: crate::local_pool
// Provides: {"woken"}
// Dependencies: {}
# [doc = " Check for a wakeup, but don't consume it."] fn woken () -> bool { CURRENT_THREAD_NOTIFY . with (| thread_notify | thread_notify . unparked . load (Ordering :: Acquire)) }
};
}
