// Generated macro for impl_645 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_645 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_645"}
// Dependencies: {}
impl Step for MirOpt { type Output = () ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . suite_path ("tests/mir-opt") } fn make_run (run : RunConfig < '_ >) { let compiler = run . builder . compiler (run . builder . top_stage , run . build_triple ()) ; run . builder . ensure (MirOpt { compiler , target : run . target }) ; } fn run (self , builder : & Builder < '_ >) { let run = | target | { builder . ensure (Compiletest { test_compiler : self . compiler , target , mode : "mir-opt" , suite : "mir-opt" , path : "tests/mir-opt" , compare_mode : None , }) } ; run (self . target) ; if builder . config . cmd . bless () { for target in ["aarch64-unknown-linux-gnu" , "i686-pc-windows-msvc"] { run (TargetSelection :: from_user (target)) ; } for target in ["x86_64-apple-darwin" , "i686-unknown-linux-musl"] { let target = TargetSelection :: from_user (target) ; let panic_abort_target = builder . ensure (MirOptPanicAbortSyntheticTarget { compiler : self . compiler , base : target , }) ; run (panic_abort_target) ; } } } }
};
}
