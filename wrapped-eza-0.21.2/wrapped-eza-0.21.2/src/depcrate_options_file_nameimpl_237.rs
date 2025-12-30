// Generated macro for impl_237 (impl)
macro_rules! Depcrate_options_file_nameimpl_237 {
() => {
// Module: crate::options::file_name
// Provides: {"impl_237"}
// Dependencies: {}
impl Classify { fn deduce (matches : & MatchedFlags < '_ >) -> Result < Self , OptionsError > { let mode_opt = matches . get (& flags :: CLASSIFY) ? ; match mode_opt { Some (word) => match word . to_str () { Some ("always") => Ok (Self :: AddFileIndicators) , Some ("auto" | "automatic") => Ok (Self :: AutomaticAddFileIndicators) , Some ("never") => Ok (Self :: JustFilenames) , _ => Err (OptionsError :: BadArgument (& flags :: CLASSIFY , word . into ())) , } , None => Ok (Self :: JustFilenames) , } } }
};
}
