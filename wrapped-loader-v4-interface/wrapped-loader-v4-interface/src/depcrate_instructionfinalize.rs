// Generated macro for finalize (function)
macro_rules! Depcrate_instructionfinalize {
() => {
// Module: crate::instruction
// Provides: {"finalize"}
// Dependencies: {}
# [doc = " Returns the instructions required to finalize program."] # [cfg (feature = "bincode")] pub fn finalize (program_address : & Pubkey , authority : & Pubkey , next_version_program_address : & Pubkey ,) -> Instruction { let accounts = vec ! [AccountMeta :: new (* program_address , false) , AccountMeta :: new_readonly (* authority , true) , AccountMeta :: new_readonly (* next_version_program_address , false) ,] ; Instruction :: new_with_bincode (id () , & LoaderV4Instruction :: Finalize , accounts) }
};
}
