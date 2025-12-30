// Generated macro for impl_254 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_254 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_254"}
// Dependencies: {}
impl Step for Clippy { type Output = Option < GeneratedTarball > ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let default = should_build_extended_tool (run . builder , "clippy") ; run . alias ("clippy") . default_condition (default) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Clippy { compilers : RustcPrivateCompilers :: new (run . builder , run . builder . top_stage , run . target) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) -> Option < GeneratedTarball > { let target = self . target ; let clippy = builder . ensure (tool :: Clippy :: from_compilers (self . compilers)) ; let cargoclippy = builder . ensure (tool :: CargoClippy :: from_compilers (self . compilers)) ; let mut tarball = Tarball :: new (builder , "clippy" , & target . triple) ; tarball . set_overlay (OverlayKind :: Clippy) ; tarball . is_preview (true) ; tarball . add_file (& clippy . tool_path , "bin" , FileType :: Executable) ; tarball . add_file (& cargoclippy . tool_path , "bin" , FileType :: Executable) ; tarball . add_legal_and_readme_to ("share/doc/clippy") ; Some (tarball . generate ()) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("clippy" , self . target) . built_by (self . compilers . build_compiler ())) } }
};
}
