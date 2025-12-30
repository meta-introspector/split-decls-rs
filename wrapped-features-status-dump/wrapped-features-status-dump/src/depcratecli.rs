// Generated macro for Cli (struct)
macro_rules! DepcrateCli {
() => {
// Module: crate
// Provides: {"Cli"}
// Dependencies: {}
# [derive (Debug , Parser)] struct Cli { # [doc = " Path to `library/` directory."] # [arg (long)] library_path : PathBuf , # [doc = " Path to `compiler/` directory."] # [arg (long)] compiler_path : PathBuf , # [doc = " Path to `output/` directory."] # [arg (long)] output_path : PathBuf , }
};
}
