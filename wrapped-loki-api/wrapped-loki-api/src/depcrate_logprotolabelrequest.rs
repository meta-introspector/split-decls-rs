// Generated macro for LabelRequest (struct)
macro_rules! Depcrate_logprotoLabelRequest {
() => {
// Module: crate::logproto
// Provides: {"LabelRequest"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct LabelRequest { # [prost (string , tag = "1")] pub name : :: prost :: alloc :: string :: String , # [doc = " True to fetch label values, false for fetch labels names."] # [prost (bool , tag = "2")] pub values : bool , # [prost (message , optional , tag = "3")] pub start : :: core :: option :: Option < :: prost_types :: Timestamp > , # [prost (message , optional , tag = "4")] pub end : :: core :: option :: Option < :: prost_types :: Timestamp > , }
};
}
