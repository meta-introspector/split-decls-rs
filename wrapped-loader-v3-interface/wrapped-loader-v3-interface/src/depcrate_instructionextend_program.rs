// Generated macro for extend_program (function)
macro_rules! Depcrate_instructionextend_program {
() => {
// Module: crate::instruction
// Provides: {"extend_program"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " Returns the instruction required to extend the size of a program's"] # [doc = " executable data account"] pub fn extend_program (program_address : & Pubkey , payer_address : Option < & Pubkey > , additional_bytes : u32 ,) -> Instruction { let program_data_address = get_program_data_address (program_address) ; let mut metas = vec ! [AccountMeta :: new (program_data_address , false) , AccountMeta :: new (* program_address , false) ,] ; if let Some (payer_address) = payer_address { metas . push (AccountMeta :: new_readonly (solana_sdk_ids :: system_program :: id () , false ,)) ; metas . push (AccountMeta :: new (* payer_address , true)) ; } Instruction :: new_with_bincode (id () , & UpgradeableLoaderInstruction :: ExtendProgram { additional_bytes } , metas ,) }
};
}
