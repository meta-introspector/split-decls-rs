// Generated macro for impl_240 (impl)
macro_rules! Depcrate_options_file_nameimpl_240 {
() => {
// Module: crate::options::file_name
// Provides: {"impl_240"}
// Dependencies: {}
impl EmbedHyperlinks { fn deduce (matches : & MatchedFlags < '_ >) -> Result < Self , OptionsError > { let flagged = matches . has (& flags :: HYPERLINK) ? ; if flagged { Ok (Self :: On) } else { Ok (Self :: Off) } } }
};
}
