// Generated macro for get_test_filter (function)
macro_rules! Depcrate_internal_test_filterget_test_filter {
() => {
// Module: crate::internal::test_filter
// Provides: {"get_test_filter"}
// Dependencies: {}
fn get_test_filter (testbridge_test_only : & str) -> Collection { let positive_negative : Vec < & str > = testbridge_test_only . splitn (2 , '-') . collect () ; let (positive_with_globs , positive_literals) : (Vec < _ > , Vec < _ >) = { let positive = positive_negative [0] ; if positive . is_empty () { (vec ! ["*"] , vec ! []) } else { positive . split (',') . partition (| s | is_glob_pattern (s)) } } ; let (negative_with_globs , negative_literals) : (Vec < _ > , Vec < _ >) = match positive_negative . get (1) { Some (negative) if ! negative . is_empty () => { negative . split (',') . partition (| s | is_glob_pattern (s)) } _ => (vec ! [] , vec ! []) , } ; Collection { positive_equals : positive_literals . into_iter () . map (| s | Equals (s . to_string ())) . collect () , positive_matches : positive_with_globs . into_iter () . map (| s | Matches (Pattern :: new (s . to_string ()))) . collect () , negative_equals : negative_literals . into_iter () . map (| s | Equals (s . to_string ())) . collect () , negative_matches : negative_with_globs . into_iter () . map (| s | Matches (Pattern :: new (s . to_string ()))) . collect () , } }
};
}
