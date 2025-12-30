// Generated macro for log (module)
macro_rules! Depcrate_plumbing_optionslog {
() => {
// Module: crate::plumbing::options
// Provides: {"log"}
// Dependencies: {}
pub mod log { use gix :: bstr :: BString ; # [doc = " List all commits in a repository, optionally limited to those that change a given path."] # [derive (Debug , clap :: Parser)] pub struct Platform { # [doc = " The git path specification to show a log for."] # [clap (value_parser = crate :: shared :: AsBString)] pub pathspec : Option < BString > , } }
};
}
