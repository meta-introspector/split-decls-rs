// Generated macro for H3PushResolved (struct)
macro_rules! Depcrate_events_h3H3PushResolved {
() => {
// Module: crate::events::h3
// Provides: {"H3PushResolved"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct H3PushResolved { push_id : Option < u64 > , stream_id : Option < u64 > , decision : Option < H3PushDecision > , }
};
}
