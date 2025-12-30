// Generated macro for inst_store_data (function)
macro_rules! Depcrate_inst_predicatesinst_store_data {
() => {
// Module: crate::inst_predicates
// Provides: {"inst_store_data"}
// Dependencies: {}
# [doc = " Get the store data, if any, from an instruction."] pub fn inst_store_data (func : & Function , inst : Inst) -> Option < Value > { match & func . dfg . insts [inst] { InstructionData :: Store { args , .. } | InstructionData :: StoreNoOffset { args , .. } => { Some (args [0]) } _ => None , } }
};
}
