// Generated macro for Config (struct)
macro_rules! Depcrate_dfa_denseConfig {
() => {
// Module: crate::dfa::dense
// Provides: {"Config"}
// Dependencies: {}
# [doc = " The configuration used for compiling a dense DFA."] # [doc = ""] # [doc = " As a convenience, [`DFA::config`] is an alias for [`Config::new`]. The"] # [doc = " advantage of the former is that it often lets you avoid importing the"] # [doc = " `Config` type directly."] # [doc = ""] # [doc = " A dense DFA configuration is a simple data object that is typically used"] # [doc = " with [`dense::Builder::configure`](self::Builder::configure)."] # [doc = ""] # [doc = " The default configuration guarantees that a search will never return"] # [doc = " a \"quit\" error, although it is possible for a search to fail if"] # [doc = " [`Config::starts_for_each_pattern`] wasn't enabled (which it is"] # [doc = " not by default) and an [`Anchored::Pattern`] mode is requested via"] # [doc = " [`Input`](crate::Input)."] # [cfg (feature = "dfa-build")] # [derive (Clone , Debug , Default)] pub struct Config { accelerate : Option < bool > , pre : Option < Option < Prefilter > > , minimize : Option < bool > , match_kind : Option < MatchKind > , start_kind : Option < StartKind > , starts_for_each_pattern : Option < bool > , byte_classes : Option < bool > , unicode_word_boundary : Option < bool > , quitset : Option < ByteSet > , specialize_start_states : Option < bool > , dfa_size_limit : Option < Option < usize > > , determinize_size_limit : Option < Option < usize > > , }
};
}
