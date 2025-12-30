// Generated macro for H3StreamType (enum)
macro_rules! Depcrate_events_h3H3StreamType {
() => {
// Module: crate::events::h3
// Provides: {"H3StreamType"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] # [serde (rename_all = "snake_case")] pub enum H3StreamType { Request , Control , Push , Reserved , # [default] Unknown , QpackEncode , QpackDecode , }
};
}
