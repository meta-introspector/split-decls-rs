// Generated macro for exclude (module)
macro_rules! Depcrate_plumbing_optionsexclude {
() => {
// Module: crate::plumbing::options
// Provides: {"exclude"}
// Dependencies: {}
pub mod exclude { use std :: ffi :: OsString ; use gix :: bstr :: BString ; use crate :: shared :: CheckPathSpec ; # [derive (Debug , clap :: Subcommand)] pub enum Subcommands { # [doc = " Check if path-specs are excluded and print the result similar to `git check-ignore`."] Query { # [doc = " Print various statistics to stderr."] # [clap (long , short = 's')] statistics : bool , # [doc = " Show actual ignore patterns instead of un-excluding an entry."] # [doc = ""] # [doc = " That way one can understand why an entry might not be excluded."] # [clap (long , short = 'i')] show_ignore_patterns : bool , # [doc = " Additional patterns to use for exclusions. They have the highest priority."] # [doc = ""] # [doc = " Useful for undoing previous patterns using the '!' prefix."] # [clap (long , short = 'p')] patterns : Vec < OsString > , # [doc = " The git path specifications to check for exclusion, or unset to read from stdin one per line."] # [clap (value_parser = CheckPathSpec)] pathspec : Vec < BString > , } , } }
};
}
