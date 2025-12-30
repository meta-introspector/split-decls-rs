// Generated macro for impl_721 (impl)
macro_rules! Depcrate_core_build_steps_toolimpl_721 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"impl_721"}
// Dependencies: {}
impl ErrorIndex { pub fn command (builder : & Builder < '_ > , compilers : RustcPrivateCompilers) -> BootstrapCommand { let mut cmd = command (builder . ensure (ErrorIndex { compilers }) . tool_path) ; let target_compiler = compilers . target_compiler () ; let mut dylib_paths = builder . rustc_lib_paths (target_compiler) ; dylib_paths . push (builder . sysroot_target_libdir (target_compiler , target_compiler . host)) ; add_dylib_path (dylib_paths , & mut cmd) ; cmd } }
};
}
