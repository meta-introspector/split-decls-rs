// Generated macro for write (function)
macro_rules! Depcrate_instructionwrite {
() => {
// Module: crate::instruction
// Provides: {"write"}
// Dependencies: {}
# [doc = " Returns the instructions required to write a chunk of program data to a"] # [doc = " buffer account."] # [cfg (feature = "bincode")] pub fn write (program_address : & Pubkey , authority : & Pubkey , offset : u32 , bytes : Vec < u8 > ,) -> Instruction { Instruction :: new_with_bincode (id () , & LoaderV4Instruction :: Write { offset , bytes } , vec ! [AccountMeta :: new (* program_address , false) , AccountMeta :: new_readonly (* authority , true) ,] ,) }
};
}
