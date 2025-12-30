// Generated macro for Utf16CharDecoder (struct)
macro_rules! Depcrate_decoding_iteratorsUtf16CharDecoder {
() => {
// Module: crate::decoding_iterators
// Provides: {"Utf16CharDecoder"}
// Dependencies: {}
# [doc = " An [`Utf16CharMerger`](struct.Utf16CharMerger.html) that also produces"] # [doc = " offsets and lengths, but can only iterate over slices."] # [doc = ""] # [doc = " See [`SliceExt::utf16char_indices()`](../trait.SliceExt.html#tymethod.utf16char_indices)"] # [doc = " for examples and error handling."] # [derive (Clone , Default)] pub struct Utf16CharDecoder < 'a > { slice : & 'a [u16] , index : usize , }
};
}
