// Generated macro for Result (struct)
macro_rules! Depcrate_statsResult {
() => {
// Module: crate::stats
// Provides: {"Result"}
// Dependencies: {}
# [doc = " Result contains LogQL query statistics."] # [derive (Clone , PartialEq , :: prost :: Message)] pub struct Result { # [prost (message , optional , tag = "1")] pub summary : :: core :: option :: Option < Summary > , # [prost (message , optional , tag = "2")] pub querier : :: core :: option :: Option < Querier > , # [prost (message , optional , tag = "3")] pub ingester : :: core :: option :: Option < Ingester > , }
};
}
