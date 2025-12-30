// Generated macro for EntryAdapter (struct)
macro_rules! Depcrate_logprotoEntryAdapter {
() => {
// Module: crate::logproto
// Provides: {"EntryAdapter"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct EntryAdapter { # [prost (message , optional , tag = "1")] pub timestamp : :: core :: option :: Option < :: prost_types :: Timestamp > , # [prost (string , tag = "2")] pub line : :: prost :: alloc :: string :: String , }
};
}
