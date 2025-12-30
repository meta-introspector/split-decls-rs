// Generated macro for deploy_from_source (function)
macro_rules! Depcrate_instructiondeploy_from_source {
() => {
// Module: crate::instruction
// Provides: {"deploy_from_source"}
// Dependencies: {}
# [doc = " Returns the instructions required to deploy a program using a buffer."] # [cfg (feature = "bincode")] pub fn deploy_from_source (program_address : & Pubkey , authority : & Pubkey , source_address : & Pubkey ,) -> Instruction { Instruction :: new_with_bincode (id () , & LoaderV4Instruction :: Deploy , vec ! [AccountMeta :: new (* program_address , false) , AccountMeta :: new_readonly (* authority , true) , AccountMeta :: new (* source_address , false) ,] ,) }
};
}
