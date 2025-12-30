// Generated macro for wake_stream (function)
macro_rules! Depcrate_connectionwake_stream {
() => {
// Module: crate::connection
// Provides: {"wake_stream"}
// Dependencies: {}
fn wake_stream (stream_id : StreamId , wakers : & mut FxHashMap < StreamId , Waker >) { if let Some (waker) = wakers . remove (& stream_id) { waker . wake () ; } }
};
}
