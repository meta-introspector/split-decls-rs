// Generated macro for Fields (struct)
macro_rules! Depcrate_ext_sliceFields {
() => {
// Module: crate::ext_slice
// Provides: {"Fields"}
// Dependencies: {}
# [doc = " An iterator over the fields in a byte string, separated by whitespace."] # [doc = ""] # [doc = " Whitespace for this iterator is defined by the Unicode property"] # [doc = " `White_Space`."] # [doc = ""] # [doc = " This iterator splits on contiguous runs of whitespace, such that the fields"] # [doc = " in `foo\\t\\t\\n  \\nbar` are `foo` and `bar`."] # [doc = ""] # [doc = " `'a` is the lifetime of the byte string being split."] # [cfg (feature = "unicode")] # [derive (Clone , Debug)] pub struct Fields < 'a > { it : FieldsWith < 'a , fn (char) -> bool > , }
};
}
