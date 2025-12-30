// Generated macro for parse_rustc_stage (function)
macro_rules! Depcrate_utils_shared_helpersparse_rustc_stage {
() => {
// Module: crate::utils::shared_helpers
// Provides: {"parse_rustc_stage"}
// Dependencies: {}
# [doc = " Parses the value of the \"RUSTC_STAGE\" environment variable and returns it as a `String`."] # [doc = " This is the stage of the *build compiler*, which we are wrapping using a rustc/rustdoc wrapper."] # [doc = ""] # [doc = " If \"RUSTC_STAGE\" was not set, the program will be terminated with 101."] pub fn parse_rustc_stage () -> u32 { env :: var ("RUSTC_STAGE") . ok () . and_then (| v | v . parse () . ok ()) . unwrap_or_else (| | { eprintln ! ("rustc shim: FATAL: RUSTC_STAGE was not set") ; eprintln ! ("rustc shim: NOTE: use `x.py build -vvv` to see all environment variables set by bootstrap") ; std :: process :: exit (101) ; }) }
};
}
