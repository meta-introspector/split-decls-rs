// Generated macro for impl_252 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_252 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_252"}
// Dependencies: {}
impl Step for RustAnalyzer { type Output = Option < GeneratedTarball > ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let default = should_build_extended_tool (run . builder , "rust-analyzer") ; run . alias ("rust-analyzer") . default_condition (default) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (RustAnalyzer { compilers : RustcPrivateCompilers :: new (run . builder , run . builder . top_stage , run . target) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) -> Option < GeneratedTarball > { let target = self . target ; let rust_analyzer = builder . ensure (tool :: RustAnalyzer :: from_compilers (self . compilers)) ; let mut tarball = Tarball :: new (builder , "rust-analyzer" , & target . triple) ; tarball . set_overlay (OverlayKind :: RustAnalyzer) ; tarball . is_preview (true) ; tarball . add_file (& rust_analyzer . tool_path , "bin" , FileType :: Executable) ; tarball . add_legal_and_readme_to ("share/doc/rust-analyzer") ; Some (tarball . generate ()) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("rust-analyzer" , self . target) . built_by (self . compilers . build_compiler ()) ,) } }
};
}
