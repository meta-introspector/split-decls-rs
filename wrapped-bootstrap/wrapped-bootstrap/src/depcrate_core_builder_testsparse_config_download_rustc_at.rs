// Generated macro for parse_config_download_rustc_at (function)
macro_rules! Depcrate_core_builder_testsparse_config_download_rustc_at {
() => {
// Module: crate::core::builder::tests
// Provides: {"parse_config_download_rustc_at"}
// Dependencies: {}
# [doc = " Parses a Config directory from `path`, with the given value of `download_rustc`."] fn parse_config_download_rustc_at (path : & Path , download_rustc : & str , ci : bool) -> Config { Config :: parse_inner (Flags :: parse (& ["build" . to_owned () , "--dry-run" . to_owned () , "--ci" . to_owned () , if ci { "true" } else { "false" } . to_owned () , format ! ("--set=rust.download-rustc='{download_rustc}'") , "--src" . to_owned () , path . to_str () . unwrap () . to_owned () ,]) , | & _ | Ok (Default :: default ()) ,) }
};
}
