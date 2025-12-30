// Generated macro for SampleQueryResponse (struct)
macro_rules! Depcrate_logprotoSampleQueryResponse {
() => {
// Module: crate::logproto
// Provides: {"SampleQueryResponse"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct SampleQueryResponse { # [prost (message , repeated , tag = "1")] pub series : :: prost :: alloc :: vec :: Vec < Series > , # [prost (message , optional , tag = "2")] pub stats : :: core :: option :: Option < super :: stats :: Ingester > , }
};
}
