// Generated macro for parse_rustc_verbose (function)
macro_rules! Depcrate_utils_shared_helpersparse_rustc_verbose {
() => {
// Module: crate::utils::shared_helpers
// Provides: {"parse_rustc_verbose"}
// Dependencies: {}
# [doc = " Parses the value of the \"RUSTC_VERBOSE\" environment variable and returns it as a `usize`."] # [doc = " If it was not defined, returns 0 by default."] # [doc = ""] # [doc = " Panics if \"RUSTC_VERBOSE\" is defined with the value that is not an unsigned integer."] pub fn parse_rustc_verbose () -> usize { match env :: var ("RUSTC_VERBOSE") { Ok (s) => usize :: from_str (& s) . expect ("RUSTC_VERBOSE should be an integer") , Err (_) => 0 , } }
};
}
