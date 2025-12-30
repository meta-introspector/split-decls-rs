// Generated macro for Match (struct)
macro_rules! DepcrateMatch {
() => {
// Module: crate
// Provides: {"Match"}
// Dependencies: {}
# [doc = " The type of a match."] # [doc = ""] # [doc = " The type of a match is a possibly empty range pointing to a contiguous"] # [doc = " block of addressable memory."] # [doc = ""] # [doc = " Every `Match` is guaranteed to satisfy the invariant that `start <= end`."] # [doc = ""] # [doc = " # Indexing"] # [doc = ""] # [doc = " This type is structurally identical to `std::ops::Range<usize>`, but"] # [doc = " is a bit more ergonomic for dealing with match indices. In particular,"] # [doc = " this type implements `Copy` and provides methods for building new `Match`"] # [doc = " values based on old `Match` values. Finally, the invariant that `start`"] # [doc = " is always less than or equal to `end` is enforced."] # [doc = ""] # [doc = " A `Match` can be used to slice a `&[u8]`, `&mut [u8]` or `&str` using"] # [doc = " range notation. e.g.,"] # [doc = ""] # [doc = " ```"] # [doc = " use grep_matcher::Match;"] # [doc = ""] # [doc = " let m = Match::new(2, 5);"] # [doc = " let bytes = b\"abcdefghi\";"] # [doc = " assert_eq!(b\"cde\", &bytes[m]);"] # [doc = " ```"] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub struct Match { start : usize , end : usize , }
};
}
