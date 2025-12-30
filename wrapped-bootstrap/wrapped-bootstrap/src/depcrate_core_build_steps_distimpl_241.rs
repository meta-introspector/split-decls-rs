// Generated macro for impl_241 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_241 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_241"}
// Dependencies: {}
impl Step for RustcDev { type Output = Option < GeneratedTarball > ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("rustc-dev") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (RustcDev :: new (run . builder , run . target)) ; } fn run (self , builder : & Builder < '_ >) -> Option < GeneratedTarball > { let build_compiler = self . build_compiler ; let target = self . target ; if skip_host_target_lib (builder , build_compiler) { return None ; } builder . ensure (compile :: Rustc :: new (build_compiler , target)) ; let tarball = Tarball :: new (builder , "rustc-dev" , & target . triple) ; let stamp = build_stamp :: librustc_stamp (builder , build_compiler , target) ; copy_target_libs (builder , target , tarball . image_dir () , & stamp) ; let src_files = & ["Cargo.lock"] ; copy_src_dirs (builder , & builder . src , & ["compiler" , "library/proc_macro"] , & [] , & tarball . image_dir () . join ("lib/rustlib/rustc-src/rust") ,) ; for file in src_files { tarball . add_file (builder . src . join (file) , "lib/rustlib/rustc-src/rust" , FileType :: Regular ,) ; } Some (tarball . generate ()) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("rustc-dev" , self . target) . built_by (self . build_compiler)) } }
};
}
