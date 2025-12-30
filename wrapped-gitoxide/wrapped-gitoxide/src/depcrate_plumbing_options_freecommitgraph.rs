// Generated macro for commitgraph (module)
macro_rules! Depcrate_plumbing_options_freecommitgraph {
() => {
// Module: crate::plumbing::options::free
// Provides: {"commitgraph"}
// Dependencies: {}
# [doc = ""] pub mod commitgraph { use std :: path :: PathBuf ; # [derive (Debug , clap :: Subcommand)] pub enum Subcommands { # [doc = " Verify the integrity of a commit graph"] Verify { # [doc = " The path to '.git/objects/info/', '.git/objects/info/commit-graphs/', or '.git/objects/info/commit-graph' to validate."] path : PathBuf , # [doc = " output statistical information about the pack"] # [clap (long , short = 's')] statistics : bool , } , } }
};
}
