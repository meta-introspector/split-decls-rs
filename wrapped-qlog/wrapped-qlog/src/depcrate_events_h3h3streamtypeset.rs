// Generated macro for H3StreamTypeSet (struct)
macro_rules! Depcrate_events_h3H3StreamTypeSet {
() => {
// Module: crate::events::h3
// Provides: {"H3StreamTypeSet"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct H3StreamTypeSet { pub owner : Option < H3Owner > , pub stream_id : u64 , pub stream_type : H3StreamType , pub stream_type_value : Option < u64 > , pub associated_push_id : Option < u64 > , }
};
}
