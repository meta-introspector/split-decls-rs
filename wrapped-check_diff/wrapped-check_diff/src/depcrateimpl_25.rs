// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl CodeFormatter for RustfmtRunner { fn format_code < 'a > (& self , code : & 'a str , config : & Option < Vec < String > > ,) -> Result < String , CheckDiffError > { let config = create_config_arg (config) ; let mut command = Command :: new (& self . binary_path) . env ("LD_LIBRARY_PATH" , & self . ld_library_path) . args (["--unstable-features" , "--skip-children" , "--emit=stdout" , config . as_str () ,]) . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) . stderr (Stdio :: piped ()) . spawn () ? ; command . stdin . as_mut () . unwrap () . write_all (code . as_bytes ()) ? ; let output = command . wait_with_output () ? ; Ok (std :: str :: from_utf8 (& output . stdout) ? . to_string ()) } }
};
}
