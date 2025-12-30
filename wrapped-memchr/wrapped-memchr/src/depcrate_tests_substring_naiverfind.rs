// Generated macro for rfind (function)
macro_rules! Depcrate_tests_substring_naiverfind {
() => {
// Module: crate::tests::substring::naive
// Provides: {"rfind"}
// Dependencies: {}
# [doc = " Naively search in reverse for the given needle in the given haystack."] pub (crate) fn rfind (haystack : & [u8] , needle : & [u8]) -> Option < usize > { let end = haystack . len () . checked_sub (needle . len ()) . map_or (0 , | i | i + 1) ; for i in (0 .. end) . rev () { if needle == & haystack [i .. i + needle . len ()] { return Some (i) ; } } None }
};
}
