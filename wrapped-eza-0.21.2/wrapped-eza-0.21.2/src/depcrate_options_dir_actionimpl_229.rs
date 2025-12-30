// Generated macro for impl_229 (impl)
macro_rules! Depcrate_options_dir_actionimpl_229 {
() => {
// Module: crate::options::dir_action
// Provides: {"impl_229"}
// Dependencies: {}
impl RecurseOptions { # [doc = " Determine which files should be recursed into, based on the `--level`"] # [doc = " flag’s value, and whether the `--tree` flag was passed, which was"] # [doc = " determined earlier. The maximum level should be a number, and this"] # [doc = " will fail with an `Err` if it isn’t."] pub fn deduce (matches : & MatchedFlags < '_ > , tree : bool) -> Result < Self , OptionsError > { if let Some (level) = matches . get (& flags :: LEVEL) ? { let arg_str = level . to_string_lossy () ; match arg_str . parse () { Ok (l) => Ok (Self { tree , max_depth : Some (l) , }) , Err (e) => { let source = NumberSource :: Arg (& flags :: LEVEL) ; Err (OptionsError :: FailedParse (arg_str . to_string () , source , e)) } } } else { Ok (Self { tree , max_depth : None , }) } } }
};
}
