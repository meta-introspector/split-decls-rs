// Generated macro for Find (struct)
macro_rules! Depcrate_ext_sliceFind {
() => {
// Module: crate::ext_slice
// Provides: {"Find"}
// Dependencies: {}
# [doc = " An iterator over non-overlapping substring matches."] # [doc = ""] # [doc = " Matches are reported by the byte offset at which they begin."] # [doc = ""] # [doc = " `'h` is the lifetime of the haystack while `'n` is the lifetime of the"] # [doc = " needle."] # [derive (Clone , Debug)] pub struct Find < 'h , 'n > { it : memmem :: FindIter < 'h , 'n > , haystack : & 'h [u8] , needle : & 'n [u8] , }
};
}
