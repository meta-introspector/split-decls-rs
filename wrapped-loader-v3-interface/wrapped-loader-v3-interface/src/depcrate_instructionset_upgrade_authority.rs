// Generated macro for set_upgrade_authority (function)
macro_rules! Depcrate_instructionset_upgrade_authority {
() => {
// Module: crate::instruction
// Provides: {"set_upgrade_authority"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " Returns the instructions required to set a program's authority."] pub fn set_upgrade_authority (program_address : & Pubkey , current_authority_address : & Pubkey , new_authority_address : Option < & Pubkey > ,) -> Instruction { let programdata_address = get_program_data_address (program_address) ; let mut metas = vec ! [AccountMeta :: new (programdata_address , false) , AccountMeta :: new_readonly (* current_authority_address , true) ,] ; if let Some (address) = new_authority_address { metas . push (AccountMeta :: new_readonly (* address , false)) ; } Instruction :: new_with_bincode (id () , & UpgradeableLoaderInstruction :: SetAuthority , metas) }
};
}
