// Generated macro for test (module)
macro_rules! Depcrate_patterntest {
() => {
// Module: crate::pattern
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: Case ; use crate :: Converter ; use super :: * ; # [cfg (feature = "random")] # [test] fn pseudo_no_triples () { let words = vec ! ["abcdefg" , "hijklmnop" , "qrstuv" , "wxyz"] ; for _ in 0 .. 5 { let new = Pattern :: PseudoRandom . mutate (& words) . join ("") ; let mut iter = new . chars () . zip (new . chars () . skip (1)) . zip (new . chars () . skip (2)) ; assert ! (! iter . clone () . any (| ((a , b) , c) | a . is_lowercase () && b . is_lowercase () && c . is_lowercase ())) ; assert ! (! iter . any (| ((a , b) , c) | a . is_uppercase () && b . is_uppercase () && c . is_uppercase ())) ; } } # [cfg (feature = "random")] # [test] fn randoms_are_random () { let words = vec ! ["abcdefg" , "hijklmnop" , "qrstuv" , "wxyz"] ; for _ in 0 .. 5 { let transformed = Pattern :: PseudoRandom . mutate (& words) ; assert_ne ! (words , transformed) ; let transformed = Pattern :: Random . mutate (& words) ; assert_ne ! (words , transformed) ; } } # [test] fn mutate_empty_strings () { for word_pattern in [lowercase_word , uppercase_word , capital_word , toggle_word] { assert_eq ! (String :: new () , word_pattern (& String :: new ())) } } # [test] fn filtering_with_custom () { let filter_camel_pattern = Pattern :: Custom (| words | { Pattern :: Camel . mutate (& words . into_iter () . filter (| word | word . len () > 0) . map (| word | * word) . collect :: < Vec < & str > > () ,) }) ; let conv = Converter :: new () . from_case (Case :: Kebab) . set_pattern (filter_camel_pattern) ; assert_eq ! (conv . convert ("--leading-delims") , "leadingDelims") ; } }
};
}
