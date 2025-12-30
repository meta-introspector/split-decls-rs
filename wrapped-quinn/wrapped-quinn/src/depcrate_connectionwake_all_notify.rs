// Generated macro for wake_all_notify (function)
macro_rules! Depcrate_connectionwake_all_notify {
() => {
// Module: crate::connection
// Provides: {"wake_all_notify"}
// Dependencies: {}
fn wake_all_notify (wakers : & mut FxHashMap < StreamId , Arc < Notify > >) { wakers . drain () . for_each (| (_ , notify) | notify . notify_waiters ()) }
};
}
