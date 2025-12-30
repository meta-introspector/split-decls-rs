// Generated macro for set_program_length (function)
macro_rules! Depcrate_instructionset_program_length {
() => {
// Module: crate::instruction
// Provides: {"set_program_length"}
// Dependencies: {}
# [doc = " Returns the instructions required to set the length of the program account."] # [cfg (feature = "bincode")] pub fn set_program_length (program_address : & Pubkey , authority : & Pubkey , new_size : u32 , recipient_address : & Pubkey ,) -> Instruction { Instruction :: new_with_bincode (id () , & LoaderV4Instruction :: SetProgramLength { new_size } , vec ! [AccountMeta :: new (* program_address , false) , AccountMeta :: new_readonly (* authority , true) , AccountMeta :: new (* recipient_address , false) ,] ,) }
};
}
