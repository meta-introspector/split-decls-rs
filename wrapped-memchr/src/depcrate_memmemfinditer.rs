// Generated macro for FindIter (struct)
macro_rules! Depcrate_memmemFindIter {
() => {
// Module: crate::memmem
// Provides: {"FindIter"}
// Dependencies: {}
# [doc = " An iterator over non-overlapping substring matches."] # [doc = ""] # [doc = " Matches are reported by the byte offset at which they begin."] # [doc = ""] # [doc = " `'h` is the lifetime of the haystack while `'n` is the lifetime of the"] # [doc = " needle."] # [derive (Debug , Clone)] pub struct FindIter < 'h , 'n > { haystack : & 'h [u8] , prestate : PrefilterState , finder : Finder < 'n > , pos : usize , }
};
}
