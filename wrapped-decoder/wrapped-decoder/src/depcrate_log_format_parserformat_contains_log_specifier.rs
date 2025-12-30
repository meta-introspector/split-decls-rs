// Generated macro for format_contains_log_specifier (function)
macro_rules! Depcrate_log_format_parserformat_contains_log_specifier {
() => {
// Module: crate::log::format::parser
// Provides: {"format_contains_log_specifier"}
// Dependencies: {}
fn format_contains_log_specifier (segments : & [LogSegment]) -> bool { for segment in segments { match & segment . metadata { LogMetadata :: Log => return true , LogMetadata :: NestedLogSegments (s) => { if format_contains_log_specifier (s) { return true ; } } _ => continue , } } false }
};
}
