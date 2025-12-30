// Generated macro for impl_256 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_256 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_256"}
// Dependencies: {}
impl Step for Miri { type Output = Option < GeneratedTarball > ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let default = should_build_extended_tool (run . builder , "miri") ; run . alias ("miri") . default_condition (default) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Miri { compilers : RustcPrivateCompilers :: new (run . builder , run . builder . top_stage , run . target) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) -> Option < GeneratedTarball > { if ! builder . build . unstable_features () { return None ; } let miri = builder . ensure (tool :: Miri :: from_compilers (self . compilers)) ; let cargomiri = builder . ensure (tool :: CargoMiri :: from_compilers (self . compilers)) ; let mut tarball = Tarball :: new (builder , "miri" , & self . target . triple) ; tarball . set_overlay (OverlayKind :: Miri) ; tarball . is_preview (true) ; tarball . add_file (& miri . tool_path , "bin" , FileType :: Executable) ; tarball . add_file (& cargomiri . tool_path , "bin" , FileType :: Executable) ; tarball . add_legal_and_readme_to ("share/doc/miri") ; Some (tarball . generate ()) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("miri" , self . target) . built_by (self . compilers . build_compiler ())) } }
};
}
