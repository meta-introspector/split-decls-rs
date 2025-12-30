// Generated macro for impl_377 (impl)
macro_rules! Depcrate_prop_nameimpl_377 {
() => {
// Module: crate::prop_name
// Provides: {"impl_377"}
// Dependencies: {}
impl PropName { # [doc = " Creates a new object from a nul-terminated string with no internal nul"] # [doc = " bytes."] # [doc = ""] # [doc = " Panics if the `value` isn’t terminated by a nul byte or contains"] # [doc = " interior nul bytes."] pub (crate) const fn new_unwrap (value : & str) -> & Self { let Some ((& 0 , bytes)) = value . as_bytes () . split_last () else { panic ! ("input was not nul-terminated") ; } ; let mut idx = 0 ; while idx < bytes . len () { assert ! (bytes [idx] != 0 , "input contained interior nul byte") ; idx += 1 ; } unsafe { let value = CStr :: from_bytes_with_nul_unchecked (value . as_bytes ()) ; & * (ptr :: from_ref :: < CStr > (value) as * const Self) } } # [doc = " Converts the value into a C string slice."] # [inline] pub fn as_c_str (& self) -> & CStr { & self . 0 } # [doc = " Converts the value into a string slice."] # [doc = ""] # [doc = " Nul byte terminating the underlying C string is not included in the"] # [doc = " returned slice."] # [inline] pub fn as_str (& self) -> & str { unsafe { std :: str :: from_utf8_unchecked (self . 0 . to_bytes ()) } } }
};
}
