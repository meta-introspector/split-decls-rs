// Generated macro for impl_239 (impl)
macro_rules! Depcrate_options_file_nameimpl_239 {
() => {
// Module: crate::options::file_name
// Provides: {"impl_239"}
// Dependencies: {}
impl QuoteStyle { pub fn deduce (matches : & MatchedFlags < '_ >) -> Result < Self , OptionsError > { if matches . has (& flags :: NO_QUOTES) ? { Ok (Self :: NoQuotes) } else { Ok (Self :: QuoteSpaces) } } }
};
}
