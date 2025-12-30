// Generated macro for merge_base (module)
macro_rules! Depcrate_plumbing_optionsmerge_base {
() => {
// Module: crate::plumbing::options
// Provides: {"merge_base"}
// Dependencies: {}
pub mod merge_base { # [derive (Debug , clap :: Parser)] # [command (about = "A command for calculating all merge-bases")] pub struct Command { # [doc = " A revspec for the first commit."] pub first : String , # [doc = " Revspecs for the other commits to compute the merge-base with."] pub others : Vec < String > , } }
};
}
