// Generated macro for freeze_lookup_table (function)
macro_rules! Depcrate_instructionfreeze_lookup_table {
() => {
// Module: crate::instruction
// Provides: {"freeze_lookup_table"}
// Dependencies: {}
# [doc = " Constructs an instruction that freezes an address lookup"] # [doc = " table so that it can never be closed or extended again. Empty"] # [doc = " lookup tables cannot be frozen."] # [cfg (feature = "bincode")] pub fn freeze_lookup_table (lookup_table_address : Pubkey , authority_address : Pubkey) -> Instruction { Instruction :: new_with_bincode (id () , & ProgramInstruction :: FreezeLookupTable , vec ! [AccountMeta :: new (lookup_table_address , false) , AccountMeta :: new_readonly (authority_address , true) ,] ,) }
};
}
