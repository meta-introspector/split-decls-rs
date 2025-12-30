// Generated macro for ToolCommands (enum)
macro_rules! Depcrate_porcelain_optionsToolCommands {
() => {
// Module: crate::porcelain::options
// Provides: {"ToolCommands"}
// Dependencies: {}
# [cfg (feature = "gitoxide-core-tools")] # [derive (Debug , clap :: Subcommand)] # [clap (subcommand_required = true)] # [clap (visible_alias = "t")] pub enum ToolCommands { # [doc = " Find all repositories in a given directory."] Find { # [doc = " If set, print additional information to help understand why the traversal is slow."] # [doc = ""] # [doc = " Typically it will encounter too many paths without a git repository, forcing a lot"] # [doc = " of additional paths to be searched unnecessarily."] # [clap (long , short = 'd')] debug : bool , # [doc = " The directory in which to find all git repositories."] # [doc = ""] # [doc = " Defaults to the current working directory."] root : Option < PathBuf > , } , # [doc = " Move all repositories found in a directory into a structure matching their clone URLs."] Organize { # [clap (long)] # [doc = " The operation will be in dry-run mode unless this flag is set."] execute : bool , # [clap (long , short = 'f')] # [doc = " The directory to use when finding input repositories to move into position."] # [doc = ""] # [doc = " Defaults to the current working directory."] repository_source : Option < PathBuf > , # [clap (long , short = 't')] # [doc = " The directory to which to move repositories found in the repository-source."] # [doc = ""] # [doc = " Defaults to the current working directory."] destination_directory : Option < PathBuf > , } , # [cfg (feature = "gitoxide-core-tools-query")] Query (tools :: Query) , EstimateHours (tools :: EstimateHours) , }
};
}
