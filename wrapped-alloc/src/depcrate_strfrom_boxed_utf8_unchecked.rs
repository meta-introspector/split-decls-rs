// Generated macro for from_boxed_utf8_unchecked (function)
macro_rules! Depcrate_strfrom_boxed_utf8_unchecked {
() => {
// Module: crate::str
// Provides: {"from_boxed_utf8_unchecked"}
// Dependencies: {}
# [doc = " Converts a boxed slice of bytes to a boxed string slice without checking"] # [doc = " that the string contains valid UTF-8."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " * The provided bytes must contain a valid UTF-8 sequence."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let smile_utf8 = Box::new([226, 152, 186]);"] # [doc = " let smile = unsafe { std::str::from_boxed_utf8_unchecked(smile_utf8) };"] # [doc = ""] # [doc = " assert_eq!(\"☺\", &*smile);"] # [doc = " ```"] # [stable (feature = "str_box_extras" , since = "1.20.0")] # [must_use] # [inline] pub unsafe fn from_boxed_utf8_unchecked (v : Box < [u8] >) -> Box < str > { unsafe { Box :: from_raw (Box :: into_raw (v) as * mut str) } }
};
}
