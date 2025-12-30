// Generated macro for impl_271 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_271 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_271"}
// Dependencies: {}
impl Step for LlvmBitcodeLinker { type Output = Option < GeneratedTarball > ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let default = should_build_extended_tool (run . builder , "llvm-bitcode-linker") ; run . alias ("llvm-bitcode-linker") . default_condition (default) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (LlvmBitcodeLinker { build_compiler : tool :: LlvmBitcodeLinker :: get_build_compiler_for_target (run . builder , run . target ,) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) -> Option < GeneratedTarball > { let target = self . target ; let llbc_linker = builder . ensure (tool :: LlvmBitcodeLinker :: from_build_compiler (self . build_compiler , target)) ; let self_contained_bin_dir = format ! ("lib/rustlib/{}/bin/self-contained" , target . triple) ; let mut tarball = Tarball :: new (builder , "llvm-bitcode-linker" , & target . triple) ; tarball . set_overlay (OverlayKind :: LlvmBitcodeLinker) ; tarball . is_preview (true) ; tarball . add_file (& llbc_linker . tool_path , self_contained_bin_dir , FileType :: Executable) ; Some (tarball . generate ()) } }
};
}
