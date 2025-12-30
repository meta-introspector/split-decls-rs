// Generated macro for impl_939 (impl)
macro_rules! Depcrate_core_builderimpl_939 {
() => {
// Module: crate::core::builder
// Provides: {"impl_939"}
// Dependencies: {}
impl Step for Libdir { type Output = PathBuf ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . never () } fn run (self , builder : & Builder < '_ >) -> PathBuf { let relative_sysroot_libdir = builder . sysroot_libdir_relative (self . compiler) ; let sysroot = builder . sysroot (self . compiler) . join (relative_sysroot_libdir) . join ("rustlib") ; if ! builder . config . dry_run () { if ! builder . download_rustc () { let sysroot_target_libdir = sysroot . join (self . target) . join ("lib") ; builder . verbose (| | { eprintln ! ("Removing sysroot {} to avoid caching bugs" , sysroot_target_libdir . display ()) }) ; let _ = fs :: remove_dir_all (& sysroot_target_libdir) ; t ! (fs :: create_dir_all (& sysroot_target_libdir)) ; } if self . compiler . stage == 0 { dist :: maybe_install_llvm_target (builder , self . compiler . host , & builder . sysroot (self . compiler) ,) ; } } sysroot } }
};
}
