// Generated macro for format_location (function)
macro_rules! Depcrate_utils_tracingformat_location {
() => {
// Module: crate::utils::tracing
// Provides: {"format_location"}
// Dependencies: {}
# [cfg (feature = "tracing")] pub fn format_location (location : std :: panic :: Location < 'static >) -> String { format ! ("{}:{}" , location . file () , location . line ()) }
};
}
