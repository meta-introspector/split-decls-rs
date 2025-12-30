// Generated macro for test_file (function)
macro_rules! Depcrate_fuzztest_file {
() => {
// Module: crate::fuzz
// Provides: {"test_file"}
// Dependencies: {}
fn test_file (source_file : & Path , remove_tmps : bool ,) -> Result < Result < () , std :: path :: PathBuf > , String > { let mut uncached = None ; test_cached (source_file , remove_tmps , & mut uncached) }
};
}
