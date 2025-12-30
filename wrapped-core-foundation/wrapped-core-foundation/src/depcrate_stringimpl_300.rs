// Generated macro for impl_300 (impl)
macro_rules! Depcrate_stringimpl_300 {
() => {
// Module: crate::string
// Provides: {"impl_300"}
// Dependencies: {}
impl CFString { # [doc = " Creates a new `CFString` instance from a Rust string."] # [inline] pub fn new (string : & str) -> CFString { unsafe { let string_ref = CFStringCreateWithBytes (kCFAllocatorDefault , string . as_ptr () , string . len () . to_CFIndex () , kCFStringEncodingUTF8 , false as Boolean ,) ; CFString :: wrap_under_create_rule (string_ref) } } # [doc = " Like `CFString::new`, but references a string that can be used as a backing store"] # [doc = " by virtue of being statically allocated."] # [inline] pub fn from_static_string (string : & 'static str) -> CFString { unsafe { let string_ref = CFStringCreateWithBytesNoCopy (kCFAllocatorDefault , string . as_ptr () , string . len () . to_CFIndex () , kCFStringEncodingUTF8 , false as Boolean , kCFAllocatorNull ,) ; TCFType :: wrap_under_create_rule (string_ref) } } # [doc = " Returns the number of characters in the string."] # [inline] pub fn char_len (& self) -> CFIndex { unsafe { CFStringGetLength (self . 0) } } }
};
}
