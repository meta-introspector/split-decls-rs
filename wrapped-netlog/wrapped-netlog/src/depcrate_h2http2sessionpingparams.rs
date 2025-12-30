// Generated macro for Http2SessionPingParams (struct)
macro_rules! Depcrate_h2Http2SessionPingParams {
() => {
// Module: crate::h2
// Provides: {"Http2SessionPingParams"}
// Dependencies: {}
# [derive (Deserialize , Debug , Default)] pub struct Http2SessionPingParams { pub is_ack : bool , # [serde (rename = "type")] pub ty : String , pub unique_id : u64 , }
};
}
