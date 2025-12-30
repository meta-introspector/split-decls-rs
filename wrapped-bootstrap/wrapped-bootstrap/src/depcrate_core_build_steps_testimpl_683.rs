// Generated macro for impl_683 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_683 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_683"}
// Dependencies: {}
impl Step for TestHelpers { type Output = () ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("tests/auxiliary/rust_test_helpers.c") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (TestHelpers { target : run . target }) } # [doc = " Compiles the `rust_test_helpers.c` library which we used in various"] # [doc = " `run-pass` tests for ABI testing."] fn run (self , builder : & Builder < '_ >) { if builder . config . dry_run () { return ; } let target = if self . target == "x86_64-fortanix-unknown-sgx" { TargetSelection :: from_user ("x86_64-unknown-linux-gnu") } else { self . target } ; let dst = builder . test_helpers_out (target) ; let src = builder . src . join ("tests/auxiliary/rust_test_helpers.c") ; if up_to_date (& src , & dst . join ("librust_test_helpers.a")) { return ; } let _guard = builder . msg_unstaged (Kind :: Build , "test helpers" , target) ; t ! (fs :: create_dir_all (& dst)) ; let mut cfg = cc :: Build :: new () ; if ! target . is_msvc () { if let Some (ar) = builder . ar (target) { cfg . archiver (ar) ; } cfg . compiler (builder . cc (target)) ; } cfg . cargo_metadata (false) . out_dir (& dst) . target (& target . triple) . host (& builder . config . host_target . triple) . opt_level (0) . warnings (false) . debug (false) . file (builder . src . join ("tests/auxiliary/rust_test_helpers.c")) . compile ("rust_test_helpers") ; } }
};
}
