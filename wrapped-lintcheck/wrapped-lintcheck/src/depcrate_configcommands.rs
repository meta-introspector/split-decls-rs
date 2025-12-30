// Generated macro for Commands (enum)
macro_rules! Depcrate_configCommands {
() => {
// Module: crate::config
// Provides: {"Commands"}
// Dependencies: {}
# [derive (Subcommand , Clone , Debug)] pub (crate) enum Commands { # [doc = " Display a markdown diff between two lintcheck log files in JSON format"] Diff { old : PathBuf , new : PathBuf , # [doc = " This will limit the number of warnings that will be printed for each lint"] # [clap (long)] truncate : bool , # [doc = " Write the diff summary to a JSON file if there are any changes"] # [clap (long , value_name = "PATH")] write_summary : Option < PathBuf > , } , # [doc = " Create a lintcheck crates TOML file containing the top N popular crates"] Popular { # [doc = " Output TOML file name"] output : PathBuf , # [doc = " Number of crate names to download"] # [clap (short , long , default_value_t = 100)] number : usize , } , }
};
}
