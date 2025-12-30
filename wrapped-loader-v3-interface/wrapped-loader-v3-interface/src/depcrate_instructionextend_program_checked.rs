// Generated macro for extend_program_checked (function)
macro_rules! Depcrate_instructionextend_program_checked {
() => {
// Module: crate::instruction
// Provides: {"extend_program_checked"}
// Dependencies: {}
# [doc = " Returns the instruction required to extend the size of a program's"] # [doc = " executable data account"] # [cfg (feature = "bincode")] pub fn extend_program_checked (program_address : & Pubkey , authority_address : & Pubkey , payer_address : Option < & Pubkey > , additional_bytes : u32 ,) -> Instruction { let program_data_address = get_program_data_address (program_address) ; let mut metas = vec ! [AccountMeta :: new (program_data_address , false) , AccountMeta :: new (* program_address , false) , AccountMeta :: new (* authority_address , true) ,] ; if let Some (payer_address) = payer_address { metas . push (AccountMeta :: new_readonly (solana_sdk_ids :: system_program :: id () , false ,)) ; metas . push (AccountMeta :: new (* payer_address , true)) ; } Instruction :: new_with_bincode (id () , & UpgradeableLoaderInstruction :: ExtendProgramChecked { additional_bytes } , metas ,) }
};
}
