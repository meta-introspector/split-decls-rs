// Generated macro for create_buffer (function)
macro_rules! Depcrate_instructioncreate_buffer {
() => {
// Module: crate::instruction
// Provides: {"create_buffer"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " Returns the instructions required to initialize a Buffer account."] pub fn create_buffer (payer_address : & Pubkey , buffer_address : & Pubkey , authority_address : & Pubkey , lamports : u64 , program_len : usize ,) -> Result < Vec < Instruction > , InstructionError > { Ok (vec ! [system_instruction :: create_account (payer_address , buffer_address , lamports , UpgradeableLoaderState :: size_of_buffer (program_len) as u64 , & id () ,) , Instruction :: new_with_bincode (id () , & UpgradeableLoaderInstruction :: InitializeBuffer , vec ! [AccountMeta :: new (* buffer_address , false) , AccountMeta :: new_readonly (* authority_address , false) ,] ,) ,]) }
};
}
