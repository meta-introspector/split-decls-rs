// Generated macro for QPackInstruction (enum)
macro_rules! Depcrate_events_qpackQPackInstruction {
() => {
// Module: crate::events::qpack
// Provides: {"QPackInstruction"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub enum QPackInstruction { SetDynamicTableCapacityInstruction { instruction_type : QpackInstructionTypeName , capacity : u64 , } , InsertWithNameReferenceInstruction { instruction_type : QpackInstructionTypeName , table_type : QpackTableType , name_index : u64 , huffman_encoded_value : bool , value_length : u64 , value : String , } , InsertWithoutNameReferenceInstruction { instruction_type : QpackInstructionTypeName , huffman_encoded_name : bool , name_length : u64 , name : String , huffman_encoded_value : bool , value_length : u64 , value : String , } , DuplicateInstruction { instruction_type : QpackInstructionTypeName , index : u64 , } , HeaderAcknowledgementInstruction { instruction_type : QpackInstructionTypeName , stream_id : String , } , StreamCancellationInstruction { instruction_type : QpackInstructionTypeName , stream_id : String , } , InsertCountIncrementInstruction { instruction_type : QpackInstructionTypeName , increment : u64 , } , }
};
}
