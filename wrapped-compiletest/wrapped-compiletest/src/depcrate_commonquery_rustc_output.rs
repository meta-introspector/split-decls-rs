// Generated macro for query_rustc_output (function)
macro_rules! Depcrate_commonquery_rustc_output {
() => {
// Module: crate::common
// Provides: {"query_rustc_output"}
// Dependencies: {}
fn query_rustc_output (config : & Config , args : & [& str] , envs : HashMap < String , String >) -> String { let query_rustc_path = config . query_rustc_path . as_deref () . unwrap_or (& config . rustc_path) ; let mut command = Command :: new (query_rustc_path) ; add_dylib_path (& mut command , iter :: once (& config . compile_lib_path)) ; command . args (& config . target_rustcflags) . args (args) ; command . env ("RUSTC_BOOTSTRAP" , "1") ; command . envs (envs) ; let output = match command . output () { Ok (output) => output , Err (e) => { fatal ! ("failed to run {command:?}: {e}") ; } } ; if ! output . status . success () { fatal ! ("failed to run {command:?}\n--- stdout\n{}\n--- stderr\n{}" , String :: from_utf8 (output . stdout) . unwrap () , String :: from_utf8 (output . stderr) . unwrap () ,) ; } String :: from_utf8 (output . stdout) . unwrap () }
};
}
