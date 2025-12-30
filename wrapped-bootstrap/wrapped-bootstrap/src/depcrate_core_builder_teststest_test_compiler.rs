// Generated macro for test_test_compiler (function)
macro_rules! Depcrate_core_builder_teststest_test_compiler {
() => {
// Module: crate::core::builder::tests
// Provides: {"test_test_compiler"}
// Dependencies: {}
# [doc = " Regression test for <https://github.com/rust-lang/rust/issues/134916>."] # [doc = ""] # [doc = " The command `./x test compiler` should invoke the step that runs unit tests"] # [doc = " for (most) compiler crates; it should not be hijacked by the cg_clif or"] # [doc = " cg_gcc tests instead."] # [test] fn test_test_compiler () { let config = configure_with_args (& ["test" , "compiler"] , & [TEST_TRIPLE_1] , & [TEST_TRIPLE_1]) ; let cache = run_build (& config . paths . clone () , config) ; let compiler = cache . contains :: < test :: CrateLibrustc > () ; let cranelift = cache . contains :: < test :: CodegenCranelift > () ; let gcc = cache . contains :: < test :: CodegenGCC > () ; assert_eq ! ((compiler , cranelift , gcc) , (true , false , false)) ; }
};
}
