// Generated macro for impl_281 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_281 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_281"}
// Dependencies: {}
impl Step for Gcc { type Output = GeneratedTarball ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("gcc") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Gcc { target : run . target }) ; } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let tarball = Tarball :: new (builder , "gcc" , & self . target . triple) ; let output = builder . ensure (super :: gcc :: Gcc { target : self . target }) ; tarball . add_file (& output . libgccjit , "lib" , FileType :: NativeLibrary) ; tarball . generate () } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("gcc" , self . target)) } }
};
}
