// Generated macro for Cli (struct)
macro_rules! DepcrateCli {
() => {
// Module: crate
// Provides: {"Cli"}
// Dependencies: {}
# [doc = " Command-line arguments for the code editor."] # [derive (Parser , Debug)] # [command (author , version , about , long_about = None)] struct Cli { # [doc = " Path to the edit job configuration file or a directory containing edit job files."] # [arg (short , long , value_name = "PATH")] config_path : PathBuf , }
};
}
