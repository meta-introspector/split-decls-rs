// Generated macro for Subcommands (enum)
macro_rules! Depcrate_plumbing_options_freeSubcommands {
() => {
// Module: crate::plumbing::options::free
// Provides: {"Subcommands"}
// Dependencies: {}
# [derive (Debug , clap :: Subcommand)] # [clap (visible_alias = "no-repo")] pub enum Subcommands { # [doc = " Subcommands for interacting with commit-graphs"] # [clap (subcommand)] CommitGraph (commitgraph :: Subcommands) , # [doc = " Subcommands for interacting with mailmaps"] Mailmap { # [clap (flatten)] cmd : mailmap :: Platform , } , # [doc = " Subcommands for interacting with pack files and indices"] # [clap (subcommand)] Pack (pack :: Subcommands) , # [doc = " Subcommands for interacting with a worktree index, typically at .git/index"] Index (index :: Platform) , # [doc = " Show information about repository discovery and when opening a repository at the current path."] Discover , }
};
}
