// Generated macro for impl_251 (impl)
macro_rules! Depcrate_options_filterimpl_251 {
() => {
// Module: crate::options::filter
// Provides: {"impl_251"}
// Dependencies: {}
impl IgnorePatterns { # [doc = " Determines the set of glob patterns to use based on the"] # [doc = " `--ignore-glob` argument’s value. This is a list of strings"] # [doc = " separated by pipe (`|`) characters, given in any order."] pub fn deduce (matches : & MatchedFlags < '_ >) -> Result < Self , OptionsError > { let Some (inputs) = matches . get (& flags :: IGNORE_GLOB) ? else { return Ok (Self :: empty ()) ; } ; let (patterns , mut errors) = Self :: parse_from_iter (inputs . to_string_lossy () . split ('|')) ; match errors . pop () { Some (e) => Err (e . into ()) , None => Ok (patterns) , } } }
};
}
