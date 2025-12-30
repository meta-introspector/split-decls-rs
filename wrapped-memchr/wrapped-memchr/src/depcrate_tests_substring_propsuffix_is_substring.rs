// Generated macro for suffix_is_substring (function)
macro_rules! Depcrate_tests_substring_propsuffix_is_substring {
() => {
// Module: crate::tests::substring::prop
// Provides: {"suffix_is_substring"}
// Dependencies: {}
# [doc = " Check that every suffix of the given byte string is a substring."] pub (crate) fn suffix_is_substring (bs : & [u8] , mut search : impl FnMut (& [u8] , & [u8]) -> Option < Option < usize > > ,) -> bool { for i in 0 .. bs . len () . saturating_sub (1) { let suffix = & bs [i ..] ; let result = match search (bs , suffix) { None => continue , Some (result) => result , } ; if ! result . is_some () { return false ; } } true }
};
}
