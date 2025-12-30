// Generated macro for impl_395 (impl)
macro_rules! Depcrate_options_helpimpl_395 {
() => {
// Module: crate::options::help
// Provides: {"impl_395"}
// Dependencies: {}
impl HelpString { # [doc = " Determines how to show help, if at all, based on the user’s"] # [doc = " command-line arguments. This one works backwards from the other"] # [doc = " ‘deduce’ functions, returning Err if help needs to be shown."] # [doc = ""] # [doc = " We don’t do any strict-mode error checking here: it’s OK to give"] # [doc = " the --help or --long flags more than once. Actually checking for"] # [doc = " errors when the user wants help is kind of petty!"] pub fn deduce (matches : & MatchedFlags < '_ >) -> Option < Self > { if matches . count (& flags :: HELP) > 0 { Some (Self) } else { None } } }
};
}
