// Generated macro for LegacySample (struct)
macro_rules! Depcrate_logprotoLegacySample {
() => {
// Module: crate::logproto
// Provides: {"LegacySample"}
// Dependencies: {}
# [doc = " LegacySample exists for backwards compatibility reasons and is deprecated. Do not use."] # [derive (Clone , PartialEq , :: prost :: Message)] pub struct LegacySample { # [prost (double , tag = "1")] pub value : f64 , # [prost (int64 , tag = "2")] pub timestamp_ms : i64 , }
};
}
