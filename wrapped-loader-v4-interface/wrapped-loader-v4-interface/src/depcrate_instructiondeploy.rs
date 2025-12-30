// Generated macro for deploy (function)
macro_rules! Depcrate_instructiondeploy {
() => {
// Module: crate::instruction
// Provides: {"deploy"}
// Dependencies: {}
# [doc = " Returns the instructions required to deploy a program."] # [cfg (feature = "bincode")] pub fn deploy (program_address : & Pubkey , authority : & Pubkey) -> Instruction { Instruction :: new_with_bincode (id () , & LoaderV4Instruction :: Deploy , vec ! [AccountMeta :: new (* program_address , false) , AccountMeta :: new_readonly (* authority , true) ,] ,) }
};
}
