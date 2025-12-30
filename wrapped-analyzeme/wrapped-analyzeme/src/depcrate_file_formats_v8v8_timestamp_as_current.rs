// Generated macro for v8_timestamp_as_current (function)
macro_rules! Depcrate_file_formats_v8v8_timestamp_as_current {
() => {
// Module: crate::file_formats::v8
// Provides: {"v8_timestamp_as_current"}
// Dependencies: {}
fn v8_timestamp_as_current (old : OldTimestamp) -> Timestamp { match old { OldTimestamp :: Interval { start , end } => Timestamp :: Interval { start , end } , OldTimestamp :: Instant (t) => Timestamp :: Instant (t) , } }
};
}
