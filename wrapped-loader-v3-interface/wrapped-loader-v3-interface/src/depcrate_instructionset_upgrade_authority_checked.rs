// Generated macro for set_upgrade_authority_checked (function)
macro_rules! Depcrate_instructionset_upgrade_authority_checked {
() => {
// Module: crate::instruction
// Provides: {"set_upgrade_authority_checked"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " Returns the instructions required to set a program's authority. If using this instruction, the new authority"] # [doc = " must sign."] pub fn set_upgrade_authority_checked (program_address : & Pubkey , current_authority_address : & Pubkey , new_authority_address : & Pubkey ,) -> Instruction { let programdata_address = get_program_data_address (program_address) ; let metas = vec ! [AccountMeta :: new (programdata_address , false) , AccountMeta :: new_readonly (* current_authority_address , true) , AccountMeta :: new_readonly (* new_authority_address , true) ,] ; Instruction :: new_with_bincode (id () , & UpgradeableLoaderInstruction :: SetAuthorityChecked , metas ,) }
};
}
