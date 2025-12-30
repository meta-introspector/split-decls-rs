// Generated macro for H3FrameCreated (struct)
macro_rules! Depcrate_events_h3H3FrameCreated {
() => {
// Module: crate::events::h3
// Provides: {"H3FrameCreated"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct H3FrameCreated { pub stream_id : u64 , pub length : Option < u64 > , pub frame : Http3Frame , pub raw : Option < RawInfo > , }
};
}
