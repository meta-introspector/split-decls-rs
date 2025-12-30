// Generated macro for SearchTest (struct)
macro_rules! Depcrate_packed_testsSearchTest {
() => {
// Module: crate::packed::tests
// Provides: {"SearchTest"}
// Dependencies: {}
# [doc = " A description of a single test against a multi-pattern searcher."] # [doc = ""] # [doc = " A single test may not necessarily pass on every configuration of a"] # [doc = " searcher. The tests are categorized and grouped appropriately below."] # [derive (Clone , Debug , Eq , PartialEq)] struct SearchTest { # [doc = " The name of this test, for debugging."] name : & 'static str , # [doc = " The patterns to search for."] patterns : & 'static [& 'static str] , # [doc = " The text to search."] haystack : & 'static str , # [doc = " Each match is a triple of (pattern_index, start, end), where"] # [doc = " pattern_index is an index into `patterns` and `start`/`end` are indices"] # [doc = " into `haystack`."] matches : & 'static [(usize , usize , usize)] , }
};
}
