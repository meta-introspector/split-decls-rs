// Generated macro for Metadata (struct)
macro_rules! DepcrateMetadata {
() => {
// Module: crate
// Provides: {"Metadata"}
// Dependencies: {}
# [derive (Clone , Debug , Deserialize)] pub struct Metadata { # [serde (deserialize_with = "system_time_from_nanos")] pub start_time : SystemTime , pub process_id : u32 , pub cmd : String , }
};
}
