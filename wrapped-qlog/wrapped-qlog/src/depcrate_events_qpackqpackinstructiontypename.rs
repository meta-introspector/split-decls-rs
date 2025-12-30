// Generated macro for QpackInstructionTypeName (enum)
macro_rules! Depcrate_events_qpackQpackInstructionTypeName {
() => {
// Module: crate::events::qpack
// Provides: {"QpackInstructionTypeName"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum QpackInstructionTypeName { SetDynamicTableCapacityInstruction , InsertWithNameReferenceInstruction , InsertWithoutNameReferenceInstruction , DuplicateInstruction , HeaderAcknowledgementInstruction , StreamCancellationInstruction , InsertCountIncrementInstruction , }
};
}
