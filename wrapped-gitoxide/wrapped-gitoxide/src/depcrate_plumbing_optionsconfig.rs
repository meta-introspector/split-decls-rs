// Generated macro for config (module)
macro_rules! Depcrate_plumbing_optionsconfig {
() => {
// Module: crate::plumbing::options
// Provides: {"config"}
// Dependencies: {}
pub mod config { use gix :: bstr :: BString ; # [doc = " Print all entries in a configuration file or access other sub-commands."] # [derive (Debug , clap :: Parser)] # [clap (subcommand_required (false))] pub struct Platform { # [doc = " The filter terms to limit the output to matching sections and subsections only."] # [doc = ""] # [doc = " Typical filters are `branch` or `remote.origin` or `remote.or*` - git-style globs are supported"] # [doc = " and comparisons are case-insensitive."] # [clap (value_parser = crate :: shared :: AsBString)] pub filter : Vec < BString > , } }
};
}
