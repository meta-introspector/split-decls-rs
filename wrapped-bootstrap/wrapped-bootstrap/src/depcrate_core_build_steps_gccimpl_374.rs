// Generated macro for impl_374 (impl)
macro_rules! Depcrate_core_build_steps_gccimpl_374 {
() => {
// Module: crate::core::build_steps::gcc
// Provides: {"impl_374"}
// Dependencies: {}
impl Step for Gcc { type Output = GccOutput ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/gcc") . alias ("gcc") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Gcc { target : run . target }) ; } # [doc = " Compile GCC (specifically `libgccjit`) for `target`."] fn run (self , builder : & Builder < '_ >) -> Self :: Output { let target = self . target ; let metadata = match get_gcc_build_status (builder , target) { GccBuildStatus :: AlreadyBuilt (path) => return GccOutput { libgccjit : path } , GccBuildStatus :: ShouldBuild (m) => m , } ; let _guard = builder . msg_unstaged (Kind :: Build , "GCC" , target) ; t ! (metadata . stamp . remove ()) ; let _time = helpers :: timeit (builder) ; let libgccjit_path = libgccjit_built_path (& metadata . install_dir) ; if builder . config . dry_run () { return GccOutput { libgccjit : libgccjit_path } ; } build_gcc (& metadata , builder , target) ; t ! (metadata . stamp . write ()) ; GccOutput { libgccjit : libgccjit_path } } }
};
}
