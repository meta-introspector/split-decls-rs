// Generated macro for QueryResponse (struct)
macro_rules! Depcrate_logprotoQueryResponse {
() => {
// Module: crate::logproto
// Provides: {"QueryResponse"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct QueryResponse { # [prost (message , repeated , tag = "1")] pub streams : :: prost :: alloc :: vec :: Vec < StreamAdapter > , # [prost (message , optional , tag = "2")] pub stats : :: core :: option :: Option < super :: stats :: Ingester > , }
};
}
