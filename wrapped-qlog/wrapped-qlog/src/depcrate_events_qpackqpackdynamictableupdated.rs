// Generated macro for QpackDynamicTableUpdated (struct)
macro_rules! Depcrate_events_qpackQpackDynamicTableUpdated {
() => {
// Module: crate::events::qpack
// Provides: {"QpackDynamicTableUpdated"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct QpackDynamicTableUpdated { pub update_type : QpackUpdateType , pub entries : Vec < QpackDynamicTableEntry > , }
};
}
