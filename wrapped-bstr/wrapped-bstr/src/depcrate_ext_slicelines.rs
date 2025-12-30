// Generated macro for Lines (struct)
macro_rules! Depcrate_ext_sliceLines {
() => {
// Module: crate::ext_slice
// Provides: {"Lines"}
// Dependencies: {}
# [doc = " An iterator over all lines in a byte string, without their terminators."] # [doc = ""] # [doc = " For this iterator, the only line terminators recognized are `\\r\\n` and"] # [doc = " `\\n`."] # [doc = ""] # [doc = " `'a` is the lifetime of the byte string being iterated over."] # [derive (Clone , Debug)] pub struct Lines < 'a > { it : LinesWithTerminator < 'a > , }
};
}
