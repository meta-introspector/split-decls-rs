// Generated macro for deploy_with_max_program_len (function)
macro_rules! Depcrate_instructiondeploy_with_max_program_len {
() => {
// Module: crate::instruction
// Provides: {"deploy_with_max_program_len"}
// Dependencies: {}
# [deprecated (since = "2.2.0" , note = "Use loader-v4 instead")] # [cfg (feature = "bincode")] # [doc = " Returns the instructions required to deploy a program with a specified"] # [doc = " maximum program length.  The maximum length must be large enough to"] # [doc = " accommodate any future upgrades."] pub fn deploy_with_max_program_len (payer_address : & Pubkey , program_address : & Pubkey , buffer_address : & Pubkey , upgrade_authority_address : & Pubkey , program_lamports : u64 , max_data_len : usize ,) -> Result < Vec < Instruction > , InstructionError > { let programdata_address = get_program_data_address (program_address) ; Ok (vec ! [system_instruction :: create_account (payer_address , program_address , program_lamports , UpgradeableLoaderState :: size_of_program () as u64 , & id () ,) , Instruction :: new_with_bincode (id () , & UpgradeableLoaderInstruction :: DeployWithMaxDataLen { max_data_len } , vec ! [AccountMeta :: new (* payer_address , true) , AccountMeta :: new (programdata_address , false) , AccountMeta :: new (* program_address , false) , AccountMeta :: new (* buffer_address , false) , AccountMeta :: new_readonly (sysvar :: rent :: id () , false) , AccountMeta :: new_readonly (sysvar :: clock :: id () , false) , AccountMeta :: new_readonly (solana_sdk_ids :: system_program :: id () , false) , AccountMeta :: new_readonly (* upgrade_authority_address , true) ,] ,) ,]) }
};
}
