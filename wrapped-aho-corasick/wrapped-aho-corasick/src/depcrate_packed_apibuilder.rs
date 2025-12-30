// Generated macro for Builder (struct)
macro_rules! Depcrate_packed_apiBuilder {
() => {
// Module: crate::packed::api
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " A builder for constructing a packed searcher from a collection of patterns."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This example shows how to use a builder to construct a searcher. By"] # [doc = " default, leftmost-first match semantics are used."] # [doc = ""] # [doc = " ```"] # [doc = " use aho_corasick::{packed::{Builder, MatchKind}, PatternID};"] # [doc = ""] # [doc = " # fn example() -> Option<()> {"] # [doc = " let searcher = Builder::new()"] # [doc = "     .add(\"foobar\")"] # [doc = "     .add(\"foo\")"] # [doc = "     .build()?;"] # [doc = " let matches: Vec<PatternID> = searcher"] # [doc = "     .find_iter(\"foobar\")"] # [doc = "     .map(|mat| mat.pattern())"] # [doc = "     .collect();"] # [doc = " assert_eq!(vec![PatternID::ZERO], matches);"] # [doc = " # Some(()) }"] # [doc = " # if cfg!(all(feature = \"std\", any("] # [doc = " #     target_arch = \"x86_64\", target_arch = \"aarch64\","] # [doc = " # ))) {"] # [doc = " #     example().unwrap()"] # [doc = " # } else {"] # [doc = " #     assert!(example().is_none());"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct Builder { # [doc = " The configuration of this builder and subsequent matcher."] config : Config , # [doc = " Set to true if the builder detects that a matcher cannot be built."] inert : bool , # [doc = " The patterns provided by the caller."] patterns : Patterns , }
};
}
