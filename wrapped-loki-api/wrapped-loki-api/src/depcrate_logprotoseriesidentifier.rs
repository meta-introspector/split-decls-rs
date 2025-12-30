// Generated macro for SeriesIdentifier (struct)
macro_rules! Depcrate_logprotoSeriesIdentifier {
() => {
// Module: crate::logproto
// Provides: {"SeriesIdentifier"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct SeriesIdentifier { # [prost (map = "string, string" , tag = "1")] pub labels : :: std :: collections :: HashMap < :: prost :: alloc :: string :: String , :: prost :: alloc :: string :: String > , }
};
}
