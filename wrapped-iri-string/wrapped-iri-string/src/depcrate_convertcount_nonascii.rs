// Generated macro for count_nonascii (function)
macro_rules! Depcrate_convertcount_nonascii {
() => {
// Module: crate::convert
// Provides: {"count_nonascii"}
// Dependencies: {}
# [doc = " Returns the number of non-ASCII characters."] # [cfg (feature = "alloc")] # [inline] # [must_use] fn count_nonascii (s : & str) -> usize { s . bytes () . filter (| b | ! b . is_ascii ()) . count () }
};
}
