// Generated macro for upgrade (function)
macro_rules! Depcrate_instructionupgrade {
() => {
// Module: crate::instruction
// Provides: {"upgrade"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " Returns the instructions required to upgrade a program."] pub fn upgrade (program_address : & Pubkey , buffer_address : & Pubkey , authority_address : & Pubkey , spill_address : & Pubkey ,) -> Instruction { let programdata_address = get_program_data_address (program_address) ; Instruction :: new_with_bincode (id () , & UpgradeableLoaderInstruction :: Upgrade , vec ! [AccountMeta :: new (programdata_address , false) , AccountMeta :: new (* program_address , false) , AccountMeta :: new (* buffer_address , false) , AccountMeta :: new (* spill_address , false) , AccountMeta :: new_readonly (sysvar :: rent :: id () , false) , AccountMeta :: new_readonly (sysvar :: clock :: id () , false) , AccountMeta :: new_readonly (* authority_address , true) ,] ,) }
};
}
