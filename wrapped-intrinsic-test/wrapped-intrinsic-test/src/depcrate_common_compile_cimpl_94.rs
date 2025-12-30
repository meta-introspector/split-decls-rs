// Generated macro for impl_94 (impl)
macro_rules! Depcrate_common_compile_cimpl_94 {
() => {
// Module: crate::common::compile_c
// Provides: {"impl_94"}
// Dependencies: {}
impl CppCompilation { pub fn command_mut (& mut self) -> & mut std :: process :: Command { & mut self . 0 } pub fn compile_object_file (& self , input : & str , output : & str ,) -> std :: io :: Result < std :: process :: Output > { let mut cmd = clone_command (& self . 0) ; cmd . args ([input , "-c" , "-o" , output]) ; cmd . output () } pub fn link_executable (& self , inputs : impl Iterator < Item = String > , output : & str ,) -> std :: io :: Result < std :: process :: Output > { let mut cmd = clone_command (& self . 0) ; cmd . args (inputs) ; cmd . args (["-o" , output]) ; cmd . output () } }
};
}
