// Generated macro for set_buffer_authority_checked (function)
macro_rules! Depcrate_instructionset_buffer_authority_checked {
() => {
// Module: crate::instruction
// Provides: {"set_buffer_authority_checked"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " Returns the instructions required to set a buffers's authority. If using this instruction, the new authority"] # [doc = " must sign."] pub fn set_buffer_authority_checked (buffer_address : & Pubkey , current_authority_address : & Pubkey , new_authority_address : & Pubkey ,) -> Instruction { Instruction :: new_with_bincode (id () , & UpgradeableLoaderInstruction :: SetAuthorityChecked , vec ! [AccountMeta :: new (* buffer_address , false) , AccountMeta :: new_readonly (* current_authority_address , true) , AccountMeta :: new_readonly (* new_authority_address , true) ,] ,) }
};
}
