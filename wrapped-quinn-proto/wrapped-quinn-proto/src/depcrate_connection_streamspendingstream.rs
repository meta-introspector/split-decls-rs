// Generated macro for PendingStream (struct)
macro_rules! Depcrate_connection_streamsPendingStream {
() => {
// Module: crate::connection::streams
// Provides: {"PendingStream"}
// Dependencies: {}
# [doc = " The [`StreamId`] of a stream with pending data queued, ordered by its priority and recency"] # [derive (Clone , PartialEq , Eq , PartialOrd , Ord)] struct PendingStream { # [doc = " The priority of the stream"] priority : i32 , # [doc = " A tie-breaker for streams of the same priority, used to improve fairness by implementing round-robin scheduling:"] # [doc = " Larger values are prioritized, so it is initialised to `u64::MAX`, and when a stream writes data, we know"] # [doc = " that it currently has the highest recency value, so it is deprioritized by setting its recency to 1 less than the"] # [doc = " previous lowest recency value, such that all other streams of this priority will get processed once before we get back"] # [doc = " round to this one"] recency : u64 , # [doc = " The ID of the stream"] id : StreamId , }
};
}
