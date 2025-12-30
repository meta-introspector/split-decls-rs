// Generated macro for impl_369 (impl)
macro_rules! Depcrate_options_viewimpl_369 {
() => {
// Module: crate::options::view
// Provides: {"impl_369"}
// Dependencies: {}
impl grid :: Options { fn deduce (matches : & MatchedFlags < '_ >) -> Result < Self , OptionsError > { let grid = grid :: Options { across : matches . has (& flags :: ACROSS) ? , } ; Ok (grid) } }
};
}
