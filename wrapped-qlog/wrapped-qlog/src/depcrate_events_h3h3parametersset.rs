// Generated macro for H3ParametersSet (struct)
macro_rules! Depcrate_events_h3H3ParametersSet {
() => {
// Module: crate::events::h3
// Provides: {"H3ParametersSet"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct H3ParametersSet { pub owner : Option < H3Owner > , # [serde (alias = "max_header_list_size")] pub max_field_section_size : Option < u64 > , pub max_table_capacity : Option < u64 > , pub blocked_streams_count : Option < u64 > , pub enable_connect_protocol : Option < u64 > , pub h3_datagram : Option < u64 > , pub waits_for_settings : Option < bool > , }
};
}
