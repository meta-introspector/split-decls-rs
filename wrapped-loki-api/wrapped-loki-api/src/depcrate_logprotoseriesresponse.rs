// Generated macro for SeriesResponse (struct)
macro_rules! Depcrate_logprotoSeriesResponse {
() => {
// Module: crate::logproto
// Provides: {"SeriesResponse"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct SeriesResponse { # [prost (message , repeated , tag = "1")] pub series : :: prost :: alloc :: vec :: Vec < SeriesIdentifier > , }
};
}
