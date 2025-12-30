// Generated macro for impl_252 (impl)
macro_rules! Depcrate_options_filterimpl_252 {
() => {
// Module: crate::options::filter
// Provides: {"impl_252"}
// Dependencies: {}
impl GitIgnore { pub fn deduce (matches : & MatchedFlags < '_ >) -> Result < Self , OptionsError > { if matches . has (& flags :: GIT_IGNORE) ? { Ok (Self :: CheckAndIgnore) } else { Ok (Self :: Off) } } }
};
}
