// Generated macro for default_output_directory (function)
macro_rules! Depcratedefault_output_directory {
() => {
// Module: crate
// Provides: {"default_output_directory"}
// Dependencies: {}
fn default_output_directory () -> & 'static PathBuf { static DEFAULT_OUTPUT_DIRECTORY : OnceLock < PathBuf > = OnceLock :: new () ; DEFAULT_OUTPUT_DIRECTORY . get_or_init (| | { if let Some (value) = env :: var_os ("CRITERION_HOME") { PathBuf :: from (value) } else if let Some (path) = cargo_target_directory () { path . join ("criterion") } else { PathBuf :: from ("target/criterion") } }) }
};
}
