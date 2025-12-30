// Generated macro for QpackEventType (enum)
macro_rules! Depcrate_events_qpackQpackEventType {
() => {
// Module: crate::events::qpack
// Provides: {"QpackEventType"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Copy , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum QpackEventType { StateUpdated , StreamStateUpdated , DynamicTableUpdated , HeadersEncoded , HeadersDecoded , InstructionCreated , InstructionParsed , }
};
}
