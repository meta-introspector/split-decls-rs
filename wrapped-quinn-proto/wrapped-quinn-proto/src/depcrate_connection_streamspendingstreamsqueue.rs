// Generated macro for PendingStreamsQueue (struct)
macro_rules! Depcrate_connection_streamsPendingStreamsQueue {
() => {
// Module: crate::connection::streams
// Provides: {"PendingStreamsQueue"}
// Dependencies: {}
# [doc = " A queue of streams with pending outgoing data, sorted by priority"] struct PendingStreamsQueue { streams : BinaryHeap < PendingStream > , # [doc = " The next stream to write out. This is `Some` when `TransportConfig::send_fairness(false)` and writing a stream is"] # [doc = " interrupted while the stream still has some pending data. See `reinsert_pending()`."] next : Option < PendingStream > , # [doc = " A monotonically decreasing counter, used to implement round-robin scheduling for streams of the same priority."] # [doc = " Underflowing is not a practical concern, as it is initialized to u64::MAX and only decremented by 1 in `push_pending`"] recency : u64 , }
};
}
