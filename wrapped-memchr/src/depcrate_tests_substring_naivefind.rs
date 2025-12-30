// Generated macro for find (function)
macro_rules! Depcrate_tests_substring_naivefind {
() => {
// Module: crate::tests::substring::naive
// Provides: {"find"}
// Dependencies: {}
# [doc = " Naively search forwards for the given needle in the given haystack."] pub (crate) fn find (haystack : & [u8] , needle : & [u8]) -> Option < usize > { let end = haystack . len () . checked_sub (needle . len ()) . map_or (0 , | i | i + 1) ; for i in 0 .. end { if needle == & haystack [i .. i + needle . len ()] { return Some (i) ; } } None }
};
}
