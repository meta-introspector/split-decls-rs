// Generated macro for activate (function)
macro_rules! Depcrate_instructionactivate {
() => {
// Module: crate::instruction
// Provides: {"activate"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " Activate a feature"] pub fn activate (feature_id : & Pubkey , funding_address : & Pubkey , rent : & Rent) -> Vec < Instruction > { activate_with_lamports (feature_id , funding_address , rent . minimum_balance (Feature :: size_of ()) ,) }
};
}
