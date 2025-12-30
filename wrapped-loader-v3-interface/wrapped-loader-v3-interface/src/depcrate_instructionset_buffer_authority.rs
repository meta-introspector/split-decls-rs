// Generated macro for set_buffer_authority (function)
macro_rules! Depcrate_instructionset_buffer_authority {
() => {
// Module: crate::instruction
// Provides: {"set_buffer_authority"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " Returns the instructions required to set a buffers's authority."] pub fn set_buffer_authority (buffer_address : & Pubkey , current_authority_address : & Pubkey , new_authority_address : & Pubkey ,) -> Instruction { Instruction :: new_with_bincode (id () , & UpgradeableLoaderInstruction :: SetAuthority , vec ! [AccountMeta :: new (* buffer_address , false) , AccountMeta :: new_readonly (* current_authority_address , true) , AccountMeta :: new_readonly (* new_authority_address , false) ,] ,) }
};
}
