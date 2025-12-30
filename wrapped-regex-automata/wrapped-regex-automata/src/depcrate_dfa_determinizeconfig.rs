// Generated macro for Config (struct)
macro_rules! Depcrate_dfa_determinizeConfig {
() => {
// Module: crate::dfa::determinize
// Provides: {"Config"}
// Dependencies: {}
# [doc = " A builder for configuring and running a DFA determinizer."] # [derive (Clone , Debug)] pub (crate) struct Config { match_kind : MatchKind , quit : ByteSet , dfa_size_limit : Option < usize > , determinize_size_limit : Option < usize > , }
};
}
