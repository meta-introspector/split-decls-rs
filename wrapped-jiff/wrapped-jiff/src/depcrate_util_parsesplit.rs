// Generated macro for split (function)
macro_rules! Depcrate_util_parsesplit {
() => {
// Module: crate::util::parse
// Provides: {"split"}
// Dependencies: {}
# [doc = " Splits the given input into two slices at the given position."] # [doc = ""] # [doc = " If the position is greater than the length of the slice given, then this"] # [doc = " returns `None`."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn split (input : & [u8] , at : usize) -> Option < (& [u8] , & [u8]) > { if at > input . len () { None } else { Some (input . split_at (at)) } }
};
}
