// Generated macro for EventSource (struct)
macro_rules! DepcrateEventSource {
() => {
// Module: crate
// Provides: {"EventSource"}
// Dependencies: {}
# [derive (Deserialize , Debug , Default)] pub struct EventSource { # [serde (skip)] pub start_time_int : u64 , pub id : i64 , pub start_time : String , # [serde (rename = "type")] pub ty : i64 , }
};
}
