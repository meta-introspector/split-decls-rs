// Generated macro for RegexOptions (struct)
macro_rules! Depcrate_re_builderRegexOptions {
() => {
// Module: crate::re_builder
// Provides: {"RegexOptions"}
// Dependencies: {}
# [doc = " The set of user configurable options for compiling zero or more regexes."] # [derive (Clone , Debug)] pub struct RegexOptions { pub pats : Vec < String > , pub size_limit : usize , pub dfa_size_limit : usize , pub case_insensitive : bool , pub multi_line : bool , pub dot_matches_new_line : bool , pub swap_greed : bool , pub ignore_whitespace : bool , pub unicode : bool , }
};
}
