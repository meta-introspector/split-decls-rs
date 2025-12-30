// Generated macro for impl_241 (impl)
macro_rules! Depcrate_options_file_nameimpl_241 {
() => {
// Module: crate::options::file_name
// Provides: {"impl_241"}
// Dependencies: {}
impl Absolute { fn deduce (matches : & MatchedFlags < '_ >) -> Result < Self , OptionsError > { match matches . get (& flags :: ABSOLUTE) ? { Some (word) => match word . to_str () { Some ("on" | "yes") => Ok (Self :: On) , Some ("follow") => Ok (Self :: Follow) , Some ("off" | "no") | None => Ok (Self :: Off) , _ => Err (OptionsError :: BadArgument (& flags :: ABSOLUTE , word . into ())) , } , None => Ok (Self :: Off) , } } }
};
}
