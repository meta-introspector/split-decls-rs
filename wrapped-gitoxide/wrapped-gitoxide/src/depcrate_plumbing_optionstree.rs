// Generated macro for tree (module)
macro_rules! Depcrate_plumbing_optionstree {
() => {
// Module: crate::plumbing::options
// Provides: {"tree"}
// Dependencies: {}
pub mod tree { # [derive (Debug , clap :: Subcommand)] pub enum Subcommands { # [doc = " Print entries in a given tree"] Entries { # [doc = " Traverse the entire tree and its subtrees respectively, not only this tree."] # [clap (long , short = 'r')] recursive : bool , # [doc = " Provide files size as well. This is expensive as the object is decoded entirely."] # [clap (long , short = 'e')] extended : bool , # [doc = " The revspec of the tree to traverse, or the tree at `HEAD` if unspecified."] treeish : Option < String > , } , # [doc = " Provide information about a tree."] Info { # [doc = " Provide files size as well. This is expensive as the object is decoded entirely."] # [clap (long , short = 'e')] extended : bool , # [doc = " The revspec of the tree to traverse, or the tree at `HEAD` if unspecified."] treeish : Option < String > , } , } }
};
}
