// Generated macro for write (function)
macro_rules! Depcrate_instructionwrite {
() => {
// Module: crate::instruction
// Provides: {"write"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " Returns the instructions required to write a chunk of program data to a"] # [doc = " buffer account."] pub fn write (buffer_address : & Pubkey , authority_address : & Pubkey , offset : u32 , bytes : Vec < u8 > ,) -> Instruction { Instruction :: new_with_bincode (id () , & UpgradeableLoaderInstruction :: Write { offset , bytes } , vec ! [AccountMeta :: new (* buffer_address , false) , AccountMeta :: new_readonly (* authority_address , true) ,] ,) }
};
}
