// Generated macro for construct_instructions_data (function)
macro_rules! Depcrateconstruct_instructions_data {
() => {
// Module: crate
// Provides: {"construct_instructions_data"}
// Dependencies: {}
# [doc = " Construct the account data for the instructions sysvar."] # [doc = ""] # [doc = " This function is used by the runtime and not available to Solana programs."] # [cfg (not (target_os = "solana"))] pub fn construct_instructions_data (instructions : & [BorrowedInstruction]) -> Vec < u8 > { let mut data = serialize_instructions (instructions) ; data . resize (data . len () + 2 , 0) ; data }
};
}
