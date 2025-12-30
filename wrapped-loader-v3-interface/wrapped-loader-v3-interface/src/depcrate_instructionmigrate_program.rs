// Generated macro for migrate_program (function)
macro_rules! Depcrate_instructionmigrate_program {
() => {
// Module: crate::instruction
// Provides: {"migrate_program"}
// Dependencies: {}
# [doc = " Returns the instructions required to migrate a program to loader-v4."] # [cfg (feature = "bincode")] pub fn migrate_program (programdata_address : & Pubkey , program_address : & Pubkey , authority : & Pubkey ,) -> Instruction { let accounts = vec ! [AccountMeta :: new (* programdata_address , false) , AccountMeta :: new (* program_address , false) , AccountMeta :: new_readonly (* authority , true) , AccountMeta :: new_readonly (loader_v4 :: id () , false) ,] ; Instruction :: new_with_bincode (id () , & UpgradeableLoaderInstruction :: Migrate , accounts) }
};
}
