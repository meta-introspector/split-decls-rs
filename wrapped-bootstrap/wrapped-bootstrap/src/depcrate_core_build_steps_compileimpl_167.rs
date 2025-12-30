// Generated macro for impl_167 (impl)
macro_rules! Depcrate_core_build_steps_compileimpl_167 {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"impl_167"}
// Dependencies: {}
impl Step for RustcLink { type Output = () ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . never () } # [doc = " Same as `std_link`, only for librustc"] fn run (self , builder : & Builder < '_ >) { let build_compiler = self . build_compiler ; let sysroot_compiler = self . sysroot_compiler ; let target = self . target ; add_to_sysroot (builder , & builder . sysroot_target_libdir (sysroot_compiler , target) , & builder . sysroot_target_libdir (sysroot_compiler , sysroot_compiler . host) , & build_stamp :: librustc_stamp (builder , build_compiler , target) ,) ; } }
};
}
