// Generated macro for RecurseOptions (struct)
macro_rules! Depcrate_fs_dir_actionRecurseOptions {
() => {
// Module: crate::fs::dir_action
// Provides: {"RecurseOptions"}
// Dependencies: {}
# [doc = " The options that determine how to recurse into a directory."] # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub struct RecurseOptions { # [doc = " Whether recursion should be done as a tree or as multiple individual"] # [doc = " views of files."] pub tree : bool , # [doc = " The maximum number of times that recursion should descend to, if one"] # [doc = " is specified."] pub max_depth : Option < usize > , }
};
}
