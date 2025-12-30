// Generated macro for deactivate_lookup_table (function)
macro_rules! Depcrate_instructiondeactivate_lookup_table {
() => {
// Module: crate::instruction
// Provides: {"deactivate_lookup_table"}
// Dependencies: {}
# [doc = " Constructs an instruction that deactivates an address lookup"] # [doc = " table so that it cannot be extended again and will be unusable"] # [doc = " and eligible for closure after a short amount of time."] # [cfg (feature = "bincode")] pub fn deactivate_lookup_table (lookup_table_address : Pubkey , authority_address : Pubkey ,) -> Instruction { Instruction :: new_with_bincode (id () , & ProgramInstruction :: DeactivateLookupTable , vec ! [AccountMeta :: new (lookup_table_address , false) , AccountMeta :: new_readonly (authority_address , true) ,] ,) }
};
}
