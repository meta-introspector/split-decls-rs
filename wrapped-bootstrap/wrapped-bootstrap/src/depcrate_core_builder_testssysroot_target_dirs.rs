// Generated macro for sysroot_target_dirs (module)
macro_rules! Depcrate_core_builder_testssysroot_target_dirs {
() => {
// Module: crate::core::builder::tests
// Provides: {"sysroot_target_dirs"}
// Dependencies: {}
mod sysroot_target_dirs { use super :: { Build , Builder , Compiler , TEST_TRIPLE_1 , TEST_TRIPLE_2 , TargetSelection , configure , } ; # [test] fn test_sysroot_target_libdir () { let build = Build :: new (configure ("build" , & [TEST_TRIPLE_1] , & [TEST_TRIPLE_1])) ; let builder = Builder :: new (& build) ; let target_triple_1 = TargetSelection :: from_user (TEST_TRIPLE_1) ; let compiler = Compiler :: new (1 , target_triple_1) ; let target_triple_2 = TargetSelection :: from_user (TEST_TRIPLE_2) ; let actual = builder . sysroot_target_libdir (compiler , target_triple_2) ; assert_eq ! (builder . sysroot (compiler) . join (builder . sysroot_libdir_relative (compiler)) . join ("rustlib") . join (TEST_TRIPLE_2) . join ("lib") , actual) ; } # [test] fn test_sysroot_target_bindir () { let build = Build :: new (configure ("build" , & [TEST_TRIPLE_1] , & [TEST_TRIPLE_1])) ; let builder = Builder :: new (& build) ; let target_triple_1 = TargetSelection :: from_user (TEST_TRIPLE_1) ; let compiler = Compiler :: new (1 , target_triple_1) ; let target_triple_2 = TargetSelection :: from_user (TEST_TRIPLE_2) ; let actual = builder . sysroot_target_bindir (compiler , target_triple_2) ; assert_eq ! (builder . sysroot (compiler) . join (builder . sysroot_libdir_relative (compiler)) . join ("rustlib") . join (TEST_TRIPLE_2) . join ("bin") , actual) ; } }
};
}
