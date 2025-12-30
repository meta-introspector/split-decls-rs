// Generated macro for Searcher (struct)
macro_rules! Depcrate_packed_apiSearcher {
() => {
// Module: crate::packed::api
// Provides: {"Searcher"}
// Dependencies: {}
# [doc = " A packed searcher for quickly finding occurrences of multiple patterns."] # [doc = ""] # [doc = " If callers need more flexible construction, or if one wants to change the"] # [doc = " match semantics (either leftmost-first or leftmost-longest), then one can"] # [doc = " use the [`Config`] and/or [`Builder`] types for more fine grained control."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This example shows how to create a searcher from an iterator of patterns."] # [doc = " By default, leftmost-first match semantics are used."] # [doc = ""] # [doc = " ```"] # [doc = " use aho_corasick::{packed::{MatchKind, Searcher}, PatternID};"] # [doc = ""] # [doc = " # fn example() -> Option<()> {"] # [doc = " let searcher = Searcher::new([\"foobar\", \"foo\"].iter().cloned())?;"] # [doc = " let matches: Vec<PatternID> = searcher"] # [doc = "     .find_iter(\"foobar\")"] # [doc = "     .map(|mat| mat.pattern())"] # [doc = "     .collect();"] # [doc = " assert_eq!(vec![PatternID::ZERO], matches);"] # [doc = " # Some(()) }"] # [doc = " # if cfg!(all(feature = \"std\", any("] # [doc = " #     target_arch = \"x86_64\", target_arch = \"aarch64\","] # [doc = " # ))) {"] # [doc = " #     example().unwrap()"] # [doc = " # } else {"] # [doc = " #     assert!(example().is_none());"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct Searcher { patterns : Arc < Patterns > , rabinkarp : RabinKarp , search_kind : SearchKind , minimum_len : usize , }
};
}
