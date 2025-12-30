// Generated macro for impl_378 (impl)
macro_rules! Depcrate_options_viewimpl_378 {
() => {
// Module: crate::options::view
// Provides: {"impl_378"}
// Dependencies: {}
impl GroupFormat { fn deduce (matches : & MatchedFlags < '_ >) -> Result < Self , OptionsError > { let flag = matches . has (& flags :: SMART_GROUP) ? ; Ok (if flag { Self :: Smart } else { Self :: Regular }) } }
};
}
