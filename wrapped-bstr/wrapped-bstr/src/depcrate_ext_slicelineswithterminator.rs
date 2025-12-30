// Generated macro for LinesWithTerminator (struct)
macro_rules! Depcrate_ext_sliceLinesWithTerminator {
() => {
// Module: crate::ext_slice
// Provides: {"LinesWithTerminator"}
// Dependencies: {}
# [doc = " An iterator over all lines in a byte string, including their terminators."] # [doc = ""] # [doc = " For this iterator, the only line terminator recognized is `\\n`. (Since"] # [doc = " line terminators are included, this also handles `\\r\\n` line endings.)"] # [doc = ""] # [doc = " Line terminators are only included if they are present in the original"] # [doc = " byte string. For example, the last line in a byte string may not end with"] # [doc = " a line terminator."] # [doc = ""] # [doc = " Concatenating all elements yielded by this iterator is guaranteed to yield"] # [doc = " the original byte string."] # [doc = ""] # [doc = " `'a` is the lifetime of the byte string being iterated over."] # [derive (Clone , Debug)] pub struct LinesWithTerminator < 'a > { bytes : & 'a [u8] , }
};
}
