// Generated macro for test (function)
macro_rules! Depcrate_fuzztest {
() => {
// Module: crate::fuzz
// Provides: {"test"}
// Dependencies: {}
# [doc = " Generates a new rustlantis file, & compares the result of running it with GCC and LLVM."] fn test (seed : u64 , print_tmp_vars : bool) -> Result < Result < () , std :: path :: PathBuf > , String > { let source_file = generate (seed , print_tmp_vars) ? ; test_file (& source_file , true) }
};
}
