// Generated macro for convert_latin1_to_str (function)
macro_rules! Depcrate_memconvert_latin1_to_str {
() => {
// Module: crate::mem
// Provides: {"convert_latin1_to_str"}
// Dependencies: {}
# [doc = " Converts bytes whose unsigned value is interpreted as Unicode code point"] # [doc = " (i.e. U+0000 to U+00FF, inclusive) to UTF-8 such that the validity of the"] # [doc = " output is signaled using the Rust type system."] # [doc = ""] # [doc = " The length of the destination buffer must be at least the length of the"] # [doc = " source buffer times two."] # [doc = ""] # [doc = " Returns the number of bytes written."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the destination buffer is shorter than stated above."] # [inline] pub fn convert_latin1_to_str (src : & [u8] , dst : & mut str) -> usize { assert ! (dst . len () >= src . len () * 2 , "Destination must not be shorter than the source times two.") ; let (read , written) = convert_latin1_to_str_partial (src , dst) ; debug_assert_eq ! (read , src . len ()) ; written }
};
}
