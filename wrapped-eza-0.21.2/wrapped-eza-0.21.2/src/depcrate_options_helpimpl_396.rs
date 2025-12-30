// Generated macro for impl_396 (impl)
macro_rules! Depcrate_options_helpimpl_396 {
() => {
// Module: crate::options::help
// Provides: {"impl_396"}
// Dependencies: {}
impl fmt :: Display for HelpString { # [doc = " Format this help options into an actual string of help"] # [doc = " text to be displayed to the user."] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { write ! (f , "{USAGE_PART1}") ? ; if cfg ! (feature = "git") { write ! (f , "\n{GIT_FILTER_HELP}") ? ; } write ! (f , "\n{USAGE_PART2}") ? ; if cfg ! (feature = "git") { write ! (f , "\n{GIT_VIEW_HELP}") ? ; } if xattr :: ENABLED { write ! (f , "\n{EXTENDED_HELP}") ? ; write ! (f , "\n{SECATTR_HELP}") ? ; } writeln ! (f) } }
};
}
