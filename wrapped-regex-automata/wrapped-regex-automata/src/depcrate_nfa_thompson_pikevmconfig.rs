// Generated macro for Config (struct)
macro_rules! Depcrate_nfa_thompson_pikevmConfig {
() => {
// Module: crate::nfa::thompson::pikevm
// Provides: {"Config"}
// Dependencies: {}
# [doc = " The configuration used for building a [`PikeVM`]."] # [doc = ""] # [doc = " A PikeVM configuration is a simple data object that is typically used with"] # [doc = " [`Builder::configure`]. It can be cheaply cloned."] # [doc = ""] # [doc = " A default configuration can be created either with `Config::new`, or"] # [doc = " perhaps more conveniently, with [`PikeVM::config`]."] # [derive (Clone , Debug , Default)] pub struct Config { match_kind : Option < MatchKind > , pre : Option < Option < Prefilter > > , }
};
}
