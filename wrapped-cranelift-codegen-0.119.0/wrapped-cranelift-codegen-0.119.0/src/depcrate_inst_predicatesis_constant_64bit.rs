// Generated macro for is_constant_64bit (function)
macro_rules! Depcrate_inst_predicatesis_constant_64bit {
() => {
// Module: crate::inst_predicates
// Provides: {"is_constant_64bit"}
// Dependencies: {}
# [doc = " Is the given instruction a constant value (`iconst`, `fconst`) that can be"] # [doc = " represented in 64 bits?"] pub fn is_constant_64bit (func : & Function , inst : Inst) -> Option < u64 > { match & func . dfg . insts [inst] { & InstructionData :: UnaryImm { imm , .. } => Some (imm . bits () as u64) , & InstructionData :: UnaryIeee16 { imm , .. } => Some (imm . bits () as u64) , & InstructionData :: UnaryIeee32 { imm , .. } => Some (imm . bits () as u64) , & InstructionData :: UnaryIeee64 { imm , .. } => Some (imm . bits ()) , _ => None , } }
};
}
