// Generated macro for convert_utf16_to_utf8 (function)
macro_rules! Depcrate_memconvert_utf16_to_utf8 {
() => {
// Module: crate::mem
// Provides: {"convert_utf16_to_utf8"}
// Dependencies: {}
# [doc = " Converts potentially-invalid UTF-16 to valid UTF-8 with errors replaced"] # [doc = " with the REPLACEMENT CHARACTER."] # [doc = ""] # [doc = " The length of the destination buffer must be at least the length of the"] # [doc = " source buffer times three."] # [doc = ""] # [doc = " Returns the number of bytes written."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the destination buffer is shorter than stated above."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If you want to convert into a `&mut str`, use `convert_utf16_to_str()`"] # [doc = " instead of using this function together with the `unsafe` method"] # [doc = " `as_bytes_mut()` on `&mut str`."] # [inline (always)] pub fn convert_utf16_to_utf8 (src : & [u16] , dst : & mut [u8]) -> usize { assert ! (dst . len () >= src . len () * 3) ; let (read , written) = convert_utf16_to_utf8_partial (src , dst) ; debug_assert_eq ! (read , src . len ()) ; written }
};
}
