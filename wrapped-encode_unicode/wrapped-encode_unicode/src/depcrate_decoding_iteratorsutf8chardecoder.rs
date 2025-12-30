// Generated macro for Utf8CharDecoder (struct)
macro_rules! Depcrate_decoding_iteratorsUtf8CharDecoder {
() => {
// Module: crate::decoding_iterators
// Provides: {"Utf8CharDecoder"}
// Dependencies: {}
# [doc = " An [`Utf8CharMerger`](struct.Utf8CharMerger.html) that also produces"] # [doc = " offsets and lengths, but can only iterate over slices."] # [doc = ""] # [doc = " See [`SliceExt::utf8char_indices()`](../trait.SliceExt.html#tymethod.utf8char_indices)"] # [doc = " for examples and error handling."] # [derive (Clone , Default)] pub struct Utf8CharDecoder < 'a > { slice : & 'a [u8] , index : usize , }
};
}
