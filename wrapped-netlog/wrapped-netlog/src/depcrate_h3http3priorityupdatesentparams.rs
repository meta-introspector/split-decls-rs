// Generated macro for Http3PriorityUpdateSentParams (struct)
macro_rules! Depcrate_h3Http3PriorityUpdateSentParams {
() => {
// Module: crate::h3
// Provides: {"Http3PriorityUpdateSentParams"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Default)] pub struct Http3PriorityUpdateSentParams { pub prioritized_element_id : u64 , pub priority_field_value : String , # [serde (rename = "type")] pub ty : Option < PrioritizedElementType > , }
};
}
