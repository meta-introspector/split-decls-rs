// Generated macro for impl_275 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_275 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_275"}
// Dependencies: {}
impl Step for Bootstrap { type Output = Option < GeneratedTarball > ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("bootstrap") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Bootstrap { target : run . target }) ; } fn run (self , builder : & Builder < '_ >) -> Option < GeneratedTarball > { let target = self . target ; let tarball = Tarball :: new (builder , "bootstrap" , & target . triple) ; let bootstrap_outdir = & builder . bootstrap_out ; for file in & ["bootstrap" , "rustc" , "rustdoc"] { tarball . add_file (bootstrap_outdir . join (exe (file , target)) , "bootstrap/bin" , FileType :: Executable ,) ; } Some (tarball . generate ()) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("bootstrap" , self . target)) } }
};
}
