// Generated macro for DroppedStream (struct)
macro_rules! Depcrate_logprotoDroppedStream {
() => {
// Module: crate::logproto
// Provides: {"DroppedStream"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct DroppedStream { # [prost (message , optional , tag = "1")] pub from : :: core :: option :: Option < :: prost_types :: Timestamp > , # [prost (message , optional , tag = "2")] pub to : :: core :: option :: Option < :: prost_types :: Timestamp > , # [prost (string , tag = "3")] pub labels : :: prost :: alloc :: string :: String , }
};
}
