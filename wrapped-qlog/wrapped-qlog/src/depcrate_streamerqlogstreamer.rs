// Generated macro for QlogStreamer (struct)
macro_rules! Depcrate_streamerQlogStreamer {
() => {
// Module: crate::streamer
// Provides: {"QlogStreamer"}
// Dependencies: {}
pub struct QlogStreamer { start_time : std :: time :: Instant , writer : Box < dyn std :: io :: Write + Send + Sync > , qlog : QlogSeq , state : StreamerState , log_level : EventImportance , }
};
}
