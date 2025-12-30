// Generated macro for SplitNReverse (struct)
macro_rules! Depcrate_ext_sliceSplitNReverse {
() => {
// Module: crate::ext_slice
// Provides: {"SplitNReverse"}
// Dependencies: {}
# [doc = " An iterator over at most `n` substrings in a byte string, split by a"] # [doc = " separator, in reverse."] # [doc = ""] # [doc = " `'h` is the lifetime of the byte string being split (the haystack), while"] # [doc = " `'s` is the lifetime of the byte string doing the splitting."] # [derive (Clone , Debug)] pub struct SplitNReverse < 'h , 's > { split : SplitReverse < 'h , 's > , limit : usize , count : usize , }
};
}
