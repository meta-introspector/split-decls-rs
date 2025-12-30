// Generated macro for impl_246 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_246 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_246"}
// Dependencies: {}
impl Step for Src { # [doc = " The output path of the src installer tarball"] type Output = GeneratedTarball ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("rust-src") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Src) ; } # [doc = " Creates the `rust-src` installer component"] fn run (self , builder : & Builder < '_ >) -> GeneratedTarball { if ! builder . config . dry_run () { builder . require_submodule ("src/llvm-project" , None) ; } let tarball = Tarball :: new_targetless (builder , "rust-src") ; let dst_src = tarball . image_dir () . join ("lib/rustlib/src/rust") ; copy_src_dirs (builder , & builder . src , & ["library" , "src/llvm-project/libunwind"] , & ["library/backtrace/crates" , "library/stdarch/Cargo.toml" , "library/stdarch/crates/stdarch-verify" , "library/stdarch/crates/intrinsic-test" ,] , & dst_src ,) ; tarball . generate () } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("src" , TargetSelection :: default ())) } }
};
}
