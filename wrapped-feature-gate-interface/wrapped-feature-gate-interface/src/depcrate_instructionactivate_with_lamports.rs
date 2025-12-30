// Generated macro for activate_with_lamports (function)
macro_rules! Depcrate_instructionactivate_with_lamports {
() => {
// Module: crate::instruction
// Provides: {"activate_with_lamports"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn activate_with_lamports (feature_id : & Pubkey , funding_address : & Pubkey , lamports : u64 ,) -> Vec < Instruction > { vec ! [system_instruction :: transfer (funding_address , feature_id , lamports) , system_instruction :: allocate (feature_id , Feature :: size_of () as u64) , system_instruction :: assign (feature_id , & id ()) ,] }
};
}
