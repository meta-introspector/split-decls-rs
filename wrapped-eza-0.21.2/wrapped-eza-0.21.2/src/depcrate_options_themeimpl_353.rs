// Generated macro for impl_353 (impl)
macro_rules! Depcrate_options_themeimpl_353 {
() => {
// Module: crate::options::theme
// Provides: {"impl_353"}
// Dependencies: {}
impl UseColours { fn deduce < V : Vars > (matches : & MatchedFlags < '_ > , vars : & V) -> Result < Self , OptionsError > { let default_value = match vars . get (vars :: NO_COLOR) { Some (_) => Self :: Never , None => Self :: Automatic , } ; let Some (word) = matches . get_where (| f | f . matches (& flags :: COLOR) || f . matches (& flags :: COLOUR)) ? else { return Ok (default_value) ; } ; if word == "always" { Ok (Self :: Always) } else if word == "auto" || word == "automatic" { Ok (Self :: Automatic) } else if word == "never" { Ok (Self :: Never) } else { Err (OptionsError :: BadArgument (& flags :: COLOR , word . into ())) } } }
};
}
