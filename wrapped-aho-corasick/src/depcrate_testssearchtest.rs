// Generated macro for SearchTest (struct)
macro_rules! Depcrate_testsSearchTest {
() => {
// Module: crate::tests
// Provides: {"SearchTest"}
// Dependencies: {}
# [doc = " A description of a single test against an Aho-Corasick automaton."] # [doc = ""] # [doc = " A single test may not necessarily pass on every configuration of an"] # [doc = " Aho-Corasick automaton. The tests are categorized and grouped appropriately"] # [doc = " below."] # [derive (Clone , Debug , Eq , PartialEq)] struct SearchTest { # [doc = " The name of this test, for debugging."] name : & 'static str , # [doc = " The patterns to search for."] patterns : & 'static [& 'static str] , # [doc = " The text to search."] haystack : & 'static str , # [doc = " Each match is a triple of (pattern_index, start, end), where"] # [doc = " pattern_index is an index into `patterns` and `start`/`end` are indices"] # [doc = " into `haystack`."] matches : & 'static [(usize , usize , usize)] , }
};
}
