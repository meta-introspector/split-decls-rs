// Generated macro for LegacyLabelPair (struct)
macro_rules! Depcrate_logprotoLegacyLabelPair {
() => {
// Module: crate::logproto
// Provides: {"LegacyLabelPair"}
// Dependencies: {}
# [doc = " LegacyLabelPair exists for backwards compatibility reasons and is deprecated. Do not use."] # [derive (Clone , PartialEq , :: prost :: Message)] pub struct LegacyLabelPair { # [prost (bytes = "vec" , tag = "1")] pub name : :: prost :: alloc :: vec :: Vec < u8 > , # [prost (bytes = "vec" , tag = "2")] pub value : :: prost :: alloc :: vec :: Vec < u8 > , }
};
}
