// Generated macro for TailRequest (struct)
macro_rules! Depcrate_logprotoTailRequest {
() => {
// Module: crate::logproto
// Provides: {"TailRequest"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct TailRequest { # [prost (string , tag = "1")] pub query : :: prost :: alloc :: string :: String , # [prost (uint32 , tag = "3")] pub delay_for : u32 , # [prost (uint32 , tag = "4")] pub limit : u32 , # [prost (message , optional , tag = "5")] pub start : :: core :: option :: Option < :: prost_types :: Timestamp > , }
};
}
