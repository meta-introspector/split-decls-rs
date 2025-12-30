// Generated macro for Config (struct)
macro_rules! Depcrate_hybrid_dfaConfig {
() => {
// Module: crate::hybrid::dfa
// Provides: {"Config"}
// Dependencies: {}
# [doc = " The configuration used for building a lazy DFA."] # [doc = ""] # [doc = " As a convenience, [`DFA::config`] is an alias for [`Config::new`]. The"] # [doc = " advantage of the former is that it often lets you avoid importing the"] # [doc = " `Config` type directly."] # [doc = ""] # [doc = " A lazy DFA configuration is a simple data object that is typically used"] # [doc = " with [`Builder::configure`]."] # [doc = ""] # [doc = " The default configuration guarantees that a search will never return a"] # [doc = " \"gave up\" or \"quit\" error, although it is possible for a search to fail"] # [doc = " if [`Config::starts_for_each_pattern`] wasn't enabled (which it is not by"] # [doc = " default) and an [`Anchored::Pattern`] mode is requested via [`Input`]."] # [derive (Clone , Debug , Default)] pub struct Config { match_kind : Option < MatchKind > , pre : Option < Option < Prefilter > > , starts_for_each_pattern : Option < bool > , byte_classes : Option < bool > , unicode_word_boundary : Option < bool > , quitset : Option < ByteSet > , specialize_start_states : Option < bool > , cache_capacity : Option < usize > , skip_cache_capacity_check : Option < bool > , minimum_cache_clear_count : Option < Option < usize > > , minimum_bytes_per_state : Option < Option < usize > > , }
};
}
