// Generated macro for ValueList (type)
macro_rules! Depcrate_ir_instructionsValueList {
() => {
// Module: crate::ir::instructions
// Provides: {"ValueList"}
// Dependencies: {}
# [doc = " Some instructions use an external list of argument values because there is not enough space in"] # [doc = " the 16-byte `InstructionData` struct. These value lists are stored in a memory pool in"] # [doc = " `dfg.value_lists`."] pub type ValueList = entity :: EntityList < Value > ;
};
}
