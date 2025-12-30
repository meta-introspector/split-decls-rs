// Generated macro for Config (struct)
macro_rules! Depcrate_configConfig {
() => {
// Module: crate::config
// Provides: {"Config"}
// Dependencies: {}
# [doc = " Config represents the configuration of a regex matcher in this crate."] # [doc = " The configuration is itself a rough combination of the knobs found in"] # [doc = " the `regex` crate itself, along with additional `grep-matcher` specific"] # [doc = " options."] # [doc = ""] # [doc = " The configuration can be used to build a \"configured\" HIR expression. A"] # [doc = " configured HIR expression is an HIR expression that is aware of the"] # [doc = " configuration which generated it, and provides transformation on that HIR"] # [doc = " such that the configuration is preserved."] # [derive (Clone , Debug)] pub (crate) struct Config { pub (crate) case_insensitive : bool , pub (crate) case_smart : bool , pub (crate) multi_line : bool , pub (crate) dot_matches_new_line : bool , pub (crate) swap_greed : bool , pub (crate) ignore_whitespace : bool , pub (crate) unicode : bool , pub (crate) octal : bool , pub (crate) size_limit : usize , pub (crate) dfa_size_limit : usize , pub (crate) nest_limit : u32 , pub (crate) line_terminator : Option < LineTerminator > , pub (crate) ban : Option < u8 > , pub (crate) crlf : bool , pub (crate) word : bool , pub (crate) fixed_strings : bool , pub (crate) whole_line : bool , }
};
}
