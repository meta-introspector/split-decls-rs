// Generated macro for impl_513 (impl)
macro_rules! Depcrate_options_stdinimpl_513 {
() => {
// Module: crate::options::stdin
// Provides: {"impl_513"}
// Dependencies: {}
impl FilesInput { pub fn deduce < V : Vars > (matches : & MatchedFlags < '_ > , vars : & V) -> Result < Self , OptionsError > { Ok (if io :: stdin () . is_terminal () || ! matches . has (& flags :: STDIN) ? { FilesInput :: Args } else if matches . has (& flags :: STDIN) ? && ! io :: stdin () . is_terminal () { let separator = vars . get (EZA_STDIN_SEPARATOR) . unwrap_or (OsString :: from ("\n")) ; FilesInput :: Stdin (separator) } else { FilesInput :: Args } ,) } }
};
}
