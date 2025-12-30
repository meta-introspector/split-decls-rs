// Generated macro for GetChunkIDsResponse (struct)
macro_rules! Depcrate_logprotoGetChunkIDsResponse {
() => {
// Module: crate::logproto
// Provides: {"GetChunkIDsResponse"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct GetChunkIDsResponse { # [prost (string , repeated , tag = "1")] pub chunk_i_ds : :: prost :: alloc :: vec :: Vec < :: prost :: alloc :: string :: String > , }
};
}
