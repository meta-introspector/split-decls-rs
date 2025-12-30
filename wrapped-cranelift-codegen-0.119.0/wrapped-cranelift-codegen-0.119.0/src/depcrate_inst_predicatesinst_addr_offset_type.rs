// Generated macro for inst_addr_offset_type (function)
macro_rules! Depcrate_inst_predicatesinst_addr_offset_type {
() => {
// Module: crate::inst_predicates
// Provides: {"inst_addr_offset_type"}
// Dependencies: {}
# [doc = " Get the address, offset, and access type from the given instruction, if any."] pub fn inst_addr_offset_type (func : & Function , inst : Inst) -> Option < (Value , Offset32 , Type) > { match & func . dfg . insts [inst] { InstructionData :: Load { arg , offset , .. } => { let ty = func . dfg . value_type (func . dfg . inst_results (inst) [0]) ; Some ((* arg , * offset , ty)) } InstructionData :: LoadNoOffset { arg , .. } => { let ty = func . dfg . value_type (func . dfg . inst_results (inst) [0]) ; Some ((* arg , 0 . into () , ty)) } InstructionData :: Store { args , offset , .. } => { let ty = func . dfg . value_type (args [0]) ; Some ((args [1] , * offset , ty)) } InstructionData :: StoreNoOffset { args , .. } => { let ty = func . dfg . value_type (args [0]) ; Some ((args [1] , 0 . into () , ty)) } _ => None , } }
};
}
