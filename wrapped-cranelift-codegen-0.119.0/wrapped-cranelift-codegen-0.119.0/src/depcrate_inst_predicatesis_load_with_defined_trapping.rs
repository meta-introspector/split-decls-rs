// Generated macro for is_load_with_defined_trapping (function)
macro_rules! Depcrate_inst_predicatesis_load_with_defined_trapping {
() => {
// Module: crate::inst_predicates
// Provides: {"is_load_with_defined_trapping"}
// Dependencies: {}
# [doc = " Load instructions without the `notrap` flag are defined to trap when"] # [doc = " operating on inaccessible memory, so we can't treat them as side-effect-free even if the loaded"] # [doc = " value is unused."] # [inline (always)] fn is_load_with_defined_trapping (opcode : Opcode , data : & InstructionData) -> bool { if ! opcode . can_load () { return false ; } match * data { InstructionData :: StackLoad { .. } => false , InstructionData :: Load { flags , .. } => ! flags . notrap () , _ => true , } }
};
}
