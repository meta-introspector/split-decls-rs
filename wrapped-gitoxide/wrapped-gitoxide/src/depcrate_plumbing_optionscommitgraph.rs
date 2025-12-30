// Generated macro for commitgraph (module)
macro_rules! Depcrate_plumbing_optionscommitgraph {
() => {
// Module: crate::plumbing::options
// Provides: {"commitgraph"}
// Dependencies: {}
# [doc = ""] pub mod commitgraph { # [derive (Debug , clap :: Subcommand)] pub enum Subcommands { # [doc = " Verify the integrity of a commit graph file."] Verify { # [doc = " output statistical information about the graph."] # [clap (long , short = 's')] statistics : bool , } , # [doc = " List all entries in the commit-graph file as reachable by starting from `HEAD`."] List { # [doc = " Display long hashes, instead of expensively shortened versions for best performance."] # [clap (long , short = 'l')] long_hashes : bool , # [doc = " The rev-spec to list reachable commits from."] # [clap (default_value = "@")] spec : std :: ffi :: OsString , } , } }
};
}
