// Generated macro for GetChunkIDsRequest (struct)
macro_rules! Depcrate_logprotoGetChunkIDsRequest {
() => {
// Module: crate::logproto
// Provides: {"GetChunkIDsRequest"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct GetChunkIDsRequest { # [prost (string , tag = "1")] pub matchers : :: prost :: alloc :: string :: String , # [prost (message , optional , tag = "2")] pub start : :: core :: option :: Option < :: prost_types :: Timestamp > , # [prost (message , optional , tag = "3")] pub end : :: core :: option :: Option < :: prost_types :: Timestamp > , }
};
}
