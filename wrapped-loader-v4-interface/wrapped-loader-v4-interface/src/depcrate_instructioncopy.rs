// Generated macro for copy (function)
macro_rules! Depcrate_instructioncopy {
() => {
// Module: crate::instruction
// Provides: {"copy"}
// Dependencies: {}
# [doc = " Returns the instructions required to copy a chunk of program data."] # [cfg (feature = "bincode")] pub fn copy (program_address : & Pubkey , authority : & Pubkey , source_address : & Pubkey , destination_offset : u32 , source_offset : u32 , length : u32 ,) -> Instruction { Instruction :: new_with_bincode (id () , & LoaderV4Instruction :: Copy { destination_offset , source_offset , length , } , vec ! [AccountMeta :: new (* program_address , false) , AccountMeta :: new_readonly (* authority , true) , AccountMeta :: new_readonly (* source_address , false) ,] ,) }
};
}
