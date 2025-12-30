// Generated macro for create_buffer (function)
macro_rules! Depcrate_instructioncreate_buffer {
() => {
// Module: crate::instruction
// Provides: {"create_buffer"}
// Dependencies: {}
# [doc = " Returns the instructions required to initialize a program/buffer account."] # [cfg (feature = "bincode")] pub fn create_buffer (payer_address : & Pubkey , buffer_address : & Pubkey , lamports : u64 , authority : & Pubkey , new_size : u32 , recipient_address : & Pubkey ,) -> Vec < Instruction > { vec ! [solana_system_interface :: instruction :: create_account (payer_address , buffer_address , lamports , 0 , & id () ,) , set_program_length (buffer_address , authority , new_size , recipient_address) ,] }
};
}
