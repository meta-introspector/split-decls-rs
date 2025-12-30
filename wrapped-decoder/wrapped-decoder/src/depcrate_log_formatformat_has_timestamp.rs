// Generated macro for format_has_timestamp (function)
macro_rules! Depcrate_log_formatformat_has_timestamp {
() => {
// Module: crate::log::format
// Provides: {"format_has_timestamp"}
// Dependencies: {}
fn format_has_timestamp (segments : & [LogSegment]) -> bool { for segment in segments { match & segment . metadata { LogMetadata :: Timestamp => return true , LogMetadata :: NestedLogSegments (s) => { if format_has_timestamp (s) { return true ; } } _ => continue , } } false }
};
}
