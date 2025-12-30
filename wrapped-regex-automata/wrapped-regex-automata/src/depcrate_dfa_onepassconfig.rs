// Generated macro for Config (struct)
macro_rules! Depcrate_dfa_onepassConfig {
() => {
// Module: crate::dfa::onepass
// Provides: {"Config"}
// Dependencies: {}
# [doc = " The configuration used for building a [one-pass DFA](DFA)."] # [doc = ""] # [doc = " A one-pass DFA configuration is a simple data object that is typically used"] # [doc = " with [`Builder::configure`]. It can be cheaply cloned."] # [doc = ""] # [doc = " A default configuration can be created either with `Config::new`, or"] # [doc = " perhaps more conveniently, with [`DFA::config`]."] # [derive (Clone , Debug , Default)] pub struct Config { match_kind : Option < MatchKind > , starts_for_each_pattern : Option < bool > , byte_classes : Option < bool > , size_limit : Option < Option < usize > > , }
};
}
