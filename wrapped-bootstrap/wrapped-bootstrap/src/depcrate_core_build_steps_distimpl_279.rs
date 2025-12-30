// Generated macro for impl_279 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_279 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_279"}
// Dependencies: {}
impl Step for ReproducibleArtifacts { type Output = Option < GeneratedTarball > ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("reproducible-artifacts") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (ReproducibleArtifacts { target : run . target }) ; } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let mut added_anything = false ; let tarball = Tarball :: new (builder , "reproducible-artifacts" , & self . target . triple) ; if let Some (path) = builder . config . rust_profile_use . as_ref () { tarball . add_file (path , "." , FileType :: Regular) ; added_anything = true ; } if let Some (path) = builder . config . llvm_profile_use . as_ref () { tarball . add_file (path , "." , FileType :: Regular) ; added_anything = true ; } for profile in & builder . config . reproducible_artifacts { tarball . add_file (profile , "." , FileType :: Regular) ; added_anything = true ; } if added_anything { Some (tarball . generate ()) } else { None } } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("reproducible-artifacts" , self . target)) } }
};
}
