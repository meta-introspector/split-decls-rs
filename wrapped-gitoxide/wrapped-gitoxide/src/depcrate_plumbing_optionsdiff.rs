// Generated macro for diff (module)
macro_rules! Depcrate_plumbing_optionsdiff {
() => {
// Module: crate::plumbing::options
// Provides: {"diff"}
// Dependencies: {}
pub mod diff { use gix :: bstr :: BString ; # [doc = " Print all changes between two objects."] # [derive (Debug , clap :: Parser)] pub struct Platform { # [clap (subcommand)] pub cmd : SubCommands , } # [derive (Debug , clap :: Subcommand)] pub enum SubCommands { # [doc = " Diff two trees."] Tree { # [doc = " A rev-spec representing the 'before' or old tree."] # [clap (value_parser = crate :: shared :: AsBString)] old_treeish : BString , # [doc = " A rev-spec representing the 'after' or new tree."] # [clap (value_parser = crate :: shared :: AsBString)] new_treeish : BString , } , # [doc = " Diff two versions of a file."] File { # [doc = " A rev-spec representing the 'before' or old state of the file, like '@~100:file'"] # [clap (value_parser = crate :: shared :: AsBString)] old_revspec : BString , # [doc = " A rev-spec representing the 'after' or new state of the file, like ':file'"] # [clap (value_parser = crate :: shared :: AsBString)] new_revspec : BString , } , } }
};
}
