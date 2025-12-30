// Generated macro for EventHeader (struct)
macro_rules! DepcrateEventHeader {
() => {
// Module: crate
// Provides: {"EventHeader"}
// Dependencies: {}
# [derive (Deserialize , Debug , Default)] pub struct EventHeader { # [serde (skip)] pub ty_string : String , # [serde (skip)] pub phase_string : String , # [serde (skip)] pub time_num : u64 , pub phase : i64 , pub source : EventSource , pub time : String , # [serde (rename = "type")] pub ty : i64 , }
};
}
