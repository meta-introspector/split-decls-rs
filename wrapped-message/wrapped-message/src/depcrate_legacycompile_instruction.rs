// Generated macro for compile_instruction (function)
macro_rules! Depcrate_legacycompile_instruction {
() => {
// Module: crate::legacy
// Provides: {"compile_instruction"}
// Dependencies: {}
fn compile_instruction (ix : & Instruction , keys : & [Address]) -> CompiledInstruction { let accounts : Vec < _ > = ix . accounts . iter () . map (| account_meta | position (keys , & account_meta . pubkey)) . collect () ; CompiledInstruction { program_id_index : position (keys , & ix . program_id) , data : ix . data . clone () , accounts , } }
};
}
