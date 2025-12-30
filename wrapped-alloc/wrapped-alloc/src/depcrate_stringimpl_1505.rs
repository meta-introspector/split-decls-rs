// Generated macro for impl_1505 (impl)
macro_rules! Depcrate_stringimpl_1505 {
() => {
// Module: crate::string
// Provides: {"impl_1505"}
// Dependencies: {}
# [unstable (feature = "string_into_chars" , issue = "133125")] impl DoubleEndedIterator for IntoChars { # [inline] fn next_back (& mut self) -> Option < char > { let len = self . as_str () . len () ; let mut iter = self . iter () ; match iter . next_back () { None => None , Some ((idx , ch)) => { let _ = self . bytes . advance_back_by (len - idx) ; Some (ch) } } } }
};
}
