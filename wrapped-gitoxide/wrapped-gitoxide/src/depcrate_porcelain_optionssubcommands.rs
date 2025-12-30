// Generated macro for Subcommands (enum)
macro_rules! Depcrate_porcelain_optionsSubcommands {
() => {
// Module: crate::porcelain::options
// Provides: {"Subcommands"}
// Dependencies: {}
# [derive (Debug , clap :: Subcommand)] pub enum Subcommands { # [doc = " Initialize the repository in the current directory."] # [clap (visible_alias = "initialize")] Init { # [doc = " The directory in which to initialize a new git repository."] # [doc = ""] # [doc = " Defaults to the current working directory."] directory : Option < PathBuf > , } , # [doc = " A selection of useful tools."] # [cfg (feature = "gitoxide-core-tools")] # [clap (subcommand)] Tool (ToolCommands) , # [doc = " Generate shell completions to stdout or a directory."] # [clap (visible_alias = "generate-completions" , visible_alias = "shell-completions")] Completions { # [doc = " The shell to generate completions for. Otherwise it's derived from the environment."] # [clap (long , short)] shell : Option < Shell > , # [doc = " The output directory in case multiple files are generated. If not provided, will write to stdout."] out_dir : Option < String > , } , # [doc = " Panic immediately, to test panic behavior."] # [cfg (debug_assertions)] Panic , }
};
}
