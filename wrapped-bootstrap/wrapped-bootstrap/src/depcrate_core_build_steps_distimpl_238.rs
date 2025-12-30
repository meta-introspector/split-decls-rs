// Generated macro for impl_238 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_238 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_238"}
// Dependencies: {}
impl Step for Std { type Output = Option < GeneratedTarball > ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("rust-std") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Std :: new (run . builder , run . target)) ; } fn run (self , builder : & Builder < '_ >) -> Option < GeneratedTarball > { let build_compiler = self . build_compiler ; let target = self . target ; if skip_host_target_lib (builder , build_compiler) { return None ; } let stamp = builder . std (build_compiler , target) . expect ("Standard library has to be built for dist") ; let mut tarball = Tarball :: new (builder , "rust-std" , & target . triple) ; tarball . include_target_in_component_name (true) ; verify_uefi_rlib_format (builder , target , & stamp) ; copy_target_libs (builder , target , tarball . image_dir () , & stamp) ; Some (tarball . generate ()) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("std" , self . target) . built_by (self . build_compiler)) } }
};
}
