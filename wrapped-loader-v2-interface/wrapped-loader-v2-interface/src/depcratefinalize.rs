// Generated macro for finalize (function)
macro_rules! Depcratefinalize {
() => {
// Module: crate
// Provides: {"finalize"}
// Dependencies: {}
# [deprecated (since = "2.2.0" , note = "Use loader-v4 instead")] # [cfg (feature = "bincode")] pub fn finalize (account_pubkey : & Pubkey , program_id : & Pubkey) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* account_pubkey , true) , AccountMeta :: new_readonly (rent :: id () , false) ,] ; Instruction :: new_with_bincode (* program_id , & LoaderInstruction :: Finalize , account_metas) }
};
}
