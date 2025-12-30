// Generated macro for TailResponse (struct)
macro_rules! Depcrate_logprotoTailResponse {
() => {
// Module: crate::logproto
// Provides: {"TailResponse"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct TailResponse { # [prost (message , optional , tag = "1")] pub stream : :: core :: option :: Option < StreamAdapter > , # [prost (message , repeated , tag = "2")] pub dropped_streams : :: prost :: alloc :: vec :: Vec < DroppedStream > , }
};
}
