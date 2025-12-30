// Generated macro for create_lookup_table_common (function)
macro_rules! Depcrate_instructioncreate_lookup_table_common {
() => {
// Module: crate::instruction
// Provides: {"create_lookup_table_common"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " Constructs an instruction to create a table account and returns"] # [doc = " the instruction and the table account's derived address."] fn create_lookup_table_common (authority_address : Pubkey , payer_address : Pubkey , recent_slot : Slot , authority_is_signer : bool ,) -> (Instruction , Pubkey) { let (lookup_table_address , bump_seed) = derive_lookup_table_address (& authority_address , recent_slot) ; let instruction = Instruction :: new_with_bincode (id () , & ProgramInstruction :: CreateLookupTable { recent_slot , bump_seed , } , vec ! [AccountMeta :: new (lookup_table_address , false) , AccountMeta :: new_readonly (authority_address , authority_is_signer) , AccountMeta :: new (payer_address , true) , AccountMeta :: new_readonly (system_program :: id () , false) ,] ,) ; (instruction , lookup_table_address) }
};
}
