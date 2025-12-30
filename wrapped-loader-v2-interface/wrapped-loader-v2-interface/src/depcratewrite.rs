// Generated macro for write (function)
macro_rules! Depcratewrite {
() => {
// Module: crate
// Provides: {"write"}
// Dependencies: {}
# [deprecated (since = "2.2.0" , note = "Use loader-v4 instead")] # [cfg (feature = "bincode")] pub fn write (account_pubkey : & Pubkey , program_id : & Pubkey , offset : u32 , bytes : Vec < u8 > ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* account_pubkey , true)] ; Instruction :: new_with_bincode (* program_id , & LoaderInstruction :: Write { offset , bytes } , account_metas ,) }
};
}
