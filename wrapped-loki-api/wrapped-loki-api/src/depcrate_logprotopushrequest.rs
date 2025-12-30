// Generated macro for PushRequest (struct)
macro_rules! Depcrate_logprotoPushRequest {
() => {
// Module: crate::logproto
// Provides: {"PushRequest"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct PushRequest { # [prost (message , repeated , tag = "1")] pub streams : :: prost :: alloc :: vec :: Vec < StreamAdapter > , }
};
}
