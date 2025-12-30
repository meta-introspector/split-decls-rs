// Generated macro for QpackStateUpdated (struct)
macro_rules! Depcrate_events_qpackQpackStateUpdated {
() => {
// Module: crate::events::qpack
// Provides: {"QpackStateUpdated"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct QpackStateUpdated { pub owner : Option < QpackOwner > , pub dynamic_table_capacity : Option < u64 > , pub dynamic_table_size : Option < u64 > , pub known_received_count : Option < u64 > , pub current_insert_count : Option < u64 > , }
};
}
