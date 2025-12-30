// Generated macro for close_lookup_table (function)
macro_rules! Depcrate_instructionclose_lookup_table {
() => {
// Module: crate::instruction
// Provides: {"close_lookup_table"}
// Dependencies: {}
# [doc = " Returns an instruction that closes an address lookup table"] # [doc = " account. The account will be deallocated and the lamports"] # [doc = " will be drained to the recipient address."] # [cfg (feature = "bincode")] pub fn close_lookup_table (lookup_table_address : Pubkey , authority_address : Pubkey , recipient_address : Pubkey ,) -> Instruction { Instruction :: new_with_bincode (id () , & ProgramInstruction :: CloseLookupTable , vec ! [AccountMeta :: new (lookup_table_address , false) , AccountMeta :: new_readonly (authority_address , true) , AccountMeta :: new (recipient_address , false) ,] ,) }
};
}
