// Generated macro for impl_277 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_277 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_277"}
// Dependencies: {}
impl Step for BuildManifest { type Output = GeneratedTarball ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("build-manifest") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (BuildManifest { target : run . target }) ; } fn run (self , builder : & Builder < '_ >) -> GeneratedTarball { let build_manifest = builder . tool_exe (Tool :: BuildManifest) ; let tarball = Tarball :: new (builder , "build-manifest" , & self . target . triple) ; tarball . add_file (& build_manifest , "bin" , FileType :: Executable) ; tarball . generate () } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("build-manifest" , self . target)) } }
};
}
