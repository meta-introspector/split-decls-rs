// Generated macro for impl_377 (impl)
macro_rules! Depcrate_options_viewimpl_377 {
() => {
// Module: crate::options::view
// Provides: {"impl_377"}
// Dependencies: {}
impl UserFormat { fn deduce (matches : & MatchedFlags < '_ >) -> Result < Self , OptionsError > { let flag = matches . has (& flags :: NUMERIC) ? ; Ok (if flag { Self :: Numeric } else { Self :: Name }) } }
};
}
