// Generated macro for wake_stream_notify (function)
macro_rules! Depcrate_connectionwake_stream_notify {
() => {
// Module: crate::connection
// Provides: {"wake_stream_notify"}
// Dependencies: {}
fn wake_stream_notify (stream_id : StreamId , wakers : & mut FxHashMap < StreamId , Arc < Notify > >) { if let Some (notify) = wakers . remove (& stream_id) { notify . notify_waiters () } }
};
}
