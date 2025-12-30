// Generated macro for H3FrameParsed (struct)
macro_rules! Depcrate_events_h3H3FrameParsed {
() => {
// Module: crate::events::h3
// Provides: {"H3FrameParsed"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct H3FrameParsed { pub stream_id : u64 , pub length : Option < u64 > , pub frame : Http3Frame , pub raw : Option < RawInfo > , }
};
}
