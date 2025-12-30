// Generated macro for impl_260 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_260 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_260"}
// Dependencies: {}
impl Step for Rustfmt { type Output = Option < GeneratedTarball > ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let default = should_build_extended_tool (run . builder , "rustfmt") ; run . alias ("rustfmt") . default_condition (default) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Rustfmt { compilers : RustcPrivateCompilers :: new (run . builder , run . builder . top_stage , run . target) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) -> Option < GeneratedTarball > { let rustfmt = builder . ensure (tool :: Rustfmt :: from_compilers (self . compilers)) ; let cargofmt = builder . ensure (tool :: Cargofmt :: from_compilers (self . compilers)) ; let mut tarball = Tarball :: new (builder , "rustfmt" , & self . target . triple) ; tarball . set_overlay (OverlayKind :: Rustfmt) ; tarball . is_preview (true) ; tarball . add_file (& rustfmt . tool_path , "bin" , FileType :: Executable) ; tarball . add_file (& cargofmt . tool_path , "bin" , FileType :: Executable) ; tarball . add_legal_and_readme_to ("share/doc/rustfmt") ; Some (tarball . generate ()) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("rustfmt" , self . target) . built_by (self . compilers . build_compiler ())) } }
};
}
