// Generated macro for ptr2str (function)
macro_rules! Depcrate_rawptr2str {
() => {
// Module: crate::raw
// Provides: {"ptr2str"}
// Dependencies: {}
# [doc = " Converts a non-empty null-terminated character string at `ptr` into a valid"] # [doc = " null-terminated UTF-8 string."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If `ptr.is_null()`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If `ptr` does not point to a null-terminated character string the behavior"] # [doc = " is undefined."] unsafe fn ptr2str (ptr : * const c_char) -> & 'static [u8] { assert ! (! ptr . is_null () , "attempt to convert a null-ptr to a UTF-8 string") ; let len = libc :: strlen (ptr) ; slice :: from_raw_parts (ptr as * const u8 , len + 1) }
};
}
