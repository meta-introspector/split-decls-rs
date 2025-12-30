// Generated macro for attributes (module)
macro_rules! Depcrate_plumbing_optionsattributes {
() => {
// Module: crate::plumbing::options
// Provides: {"attributes"}
// Dependencies: {}
pub mod attributes { use gix :: bstr :: BString ; use crate :: shared :: CheckPathSpec ; # [derive (Debug , clap :: Subcommand)] pub enum Subcommands { # [doc = " Run `git check-attr` and `git check-ignore` on all files of the index or all files"] # [doc = " passed via stdin and validate that we get the same outcome when computing attributes."] ValidateBaseline { # [doc = " Print various statistics to stderr."] # [clap (long , short = 's')] statistics : bool , # [doc = " Don't validated excludes as obtaining them with `check-ignore` can be very slow."] # [clap (long)] no_ignore : bool , } , # [doc = " List all attributes of the given path-specs and display the result similar to `git check-attr`."] Query { # [doc = " Print various statistics to stderr."] # [clap (long , short = 's')] statistics : bool , # [doc = " The Git path specifications to list attributes for, or unset to read from stdin one per line."] # [clap (value_parser = CheckPathSpec)] pathspec : Vec < BString > , } , } }
};
}
