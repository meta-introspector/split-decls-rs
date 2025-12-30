// Generated macro for extend_lookup_table (function)
macro_rules! Depcrate_instructionextend_lookup_table {
() => {
// Module: crate::instruction
// Provides: {"extend_lookup_table"}
// Dependencies: {}
# [doc = " Constructs an instruction which extends an address lookup"] # [doc = " table account with new addresses."] # [cfg (feature = "bincode")] pub fn extend_lookup_table (lookup_table_address : Pubkey , authority_address : Pubkey , payer_address : Option < Pubkey > , new_addresses : Vec < Pubkey > ,) -> Instruction { let mut accounts = vec ! [AccountMeta :: new (lookup_table_address , false) , AccountMeta :: new_readonly (authority_address , true) ,] ; if let Some (payer_address) = payer_address { accounts . extend ([AccountMeta :: new (payer_address , true) , AccountMeta :: new_readonly (system_program :: id () , false) ,]) ; } Instruction :: new_with_bincode (id () , & ProgramInstruction :: ExtendLookupTable { new_addresses } , accounts ,) }
};
}
