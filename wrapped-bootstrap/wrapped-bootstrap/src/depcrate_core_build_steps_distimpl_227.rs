// Generated macro for impl_227 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_227 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_227"}
// Dependencies: {}
impl Step for Mingw { type Output = Option < GeneratedTarball > ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("rust-mingw") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Mingw { target : run . target }) ; } fn run (self , builder : & Builder < '_ >) -> Option < GeneratedTarball > { let target = self . target ; if ! target . ends_with ("pc-windows-gnu") || ! builder . config . dist_include_mingw_linker { return None ; } let mut tarball = Tarball :: new (builder , "rust-mingw" , & target . triple) ; tarball . set_product_name ("Rust MinGW") ; make_win_dist (tarball . image_dir () , target , builder) ; Some (tarball . generate ()) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("mingw" , self . target)) } }
};
}
