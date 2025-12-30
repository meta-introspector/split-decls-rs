// Generated macro for Seed (struct)
macro_rules! Depcrate_tests_memchrSeed {
() => {
// Module: crate::tests::memchr
// Provides: {"Seed"}
// Dependencies: {}
# [doc = " Data that can be expanded into many memchr tests by padding out the corpus."] # [derive (Clone , Debug)] struct Seed { # [doc = " The thing to search. We use `&str` instead of `&[u8]` because they"] # [doc = " are nicer to write in tests, and we don't miss much since memchr"] # [doc = " doesn't care about UTF-8."] # [doc = ""] # [doc = " Corpora cannot contain either '%' or '#'. We use these bytes when"] # [doc = " expanding test cases into many test cases, and we assume they are not"] # [doc = " used. If they are used, `memchr_tests` will panic."] haystack : & 'static str , # [doc = " The needles to search for. This is intended to be an alternation of"] # [doc = " needles. The number of needles may cause this test to be skipped for"] # [doc = " some memchr variants. For example, a test with 2 needles cannot be used"] # [doc = " to test `memchr`, but can be used to test `memchr2` and `memchr3`."] # [doc = " However, a test with only 1 needle can be used to test all of `memchr`,"] # [doc = " `memchr2` and `memchr3`. We achieve this by filling in the needles with"] # [doc = " bytes that we never used in the corpus (such as '#')."] needles : & 'static [u8] , # [doc = " The positions expected to match for all of the needles."] positions : & 'static [usize] , }
};
}
