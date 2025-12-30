// Generated macro for impl_250 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_250 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_250"}
// Dependencies: {}
impl Step for Cargo { type Output = Option < GeneratedTarball > ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let default = should_build_extended_tool (run . builder , "cargo") ; run . alias ("cargo") . default_condition (default) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Cargo { build_compiler : get_tool_target_compiler (run . builder , ToolTargetBuildMode :: Build (run . target) ,) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) -> Option < GeneratedTarball > { let build_compiler = self . build_compiler ; let target = self . target ; let cargo = builder . ensure (tool :: Cargo :: from_build_compiler (build_compiler , target)) ; let src = builder . src . join ("src/tools/cargo") ; let etc = src . join ("src/etc") ; let mut tarball = Tarball :: new (builder , "cargo" , & target . triple) ; tarball . set_overlay (OverlayKind :: Cargo) ; tarball . add_file (& cargo . tool_path , "bin" , FileType :: Executable) ; tarball . add_file (etc . join ("_cargo") , "share/zsh/site-functions" , FileType :: Regular) ; tarball . add_renamed_file (etc . join ("cargo.bashcomp.sh") , "etc/bash_completion.d" , "cargo" , FileType :: Regular ,) ; tarball . add_dir (etc . join ("man") , "share/man/man1") ; tarball . add_legal_and_readme_to ("share/doc/cargo") ; Some (tarball . generate ()) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("cargo" , self . target) . built_by (self . build_compiler)) } }
};
}
