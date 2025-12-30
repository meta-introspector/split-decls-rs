// Generated macro for Bytes (struct)
macro_rules! Depcrate_ext_sliceBytes {
() => {
// Module: crate::ext_slice
// Provides: {"Bytes"}
// Dependencies: {}
# [doc = " An iterator over the bytes in a byte string."] # [doc = ""] # [doc = " `'a` is the lifetime of the byte string being traversed."] # [derive (Clone , Debug)] pub struct Bytes < 'a > { it : slice :: Iter < 'a , u8 > , }
};
}
