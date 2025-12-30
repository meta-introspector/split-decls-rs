// Generated macro for lintcheck_test (function)
macro_rules! Depcratelintcheck_test {
() => {
// Module: crate
// Provides: {"lintcheck_test"}
// Dependencies: {}
# [test] fn lintcheck_test () { let args = ["run" , "--target-dir" , "lintcheck/target" , "--manifest-path" , "./lintcheck/Cargo.toml" , "--" , "--crates-toml" , "lintcheck/test_sources.toml" ,] ; let status = Command :: new ("cargo") . args (args) . current_dir ("..") . status () ; assert ! (status . unwrap () . success ()) ; }
};
}
