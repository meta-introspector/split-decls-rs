// Generated macro for impl_120 (impl)
macro_rules! Depcrate_dfa_regeximpl_120 {
() => {
// Module: crate::dfa::regex
// Provides: {"impl_120"}
// Dependencies: {}
# [doc = " Convenience routines for regex construction."] impl Regex < dense :: DFA < & 'static [u32] > > { # [doc = " Return a builder for configuring the construction of a `Regex`."] # [doc = ""] # [doc = " This is a convenience routine to avoid needing to import the"] # [doc = " [`Builder`] type in common cases."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This example shows how to use the builder to disable UTF-8 mode"] # [doc = " everywhere."] # [doc = ""] # [doc = " ```"] # [doc = " # if cfg!(miri) { return Ok(()); } // miri takes too long"] # [doc = " use regex_automata::{"] # [doc = "     dfa::regex::Regex, nfa::thompson, util::syntax, Match,"] # [doc = " };"] # [doc = ""] # [doc = " let re = Regex::builder()"] # [doc = "     .syntax(syntax::Config::new().utf8(false))"] # [doc = "     .thompson(thompson::Config::new().utf8(false))"] # [doc = "     .build(r\"foo(?-u:[^b])ar.*\")?;"] # [doc = " let haystack = b\"\\xFEfoo\\xFFarzz\\xE2\\x98\\xFF\\n\";"] # [doc = " let expected = Some(Match::must(0, 1..9));"] # [doc = " let got = re.find(haystack);"] # [doc = " assert_eq!(expected, got);"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn builder () -> Builder { Builder :: new () } }
};
}
