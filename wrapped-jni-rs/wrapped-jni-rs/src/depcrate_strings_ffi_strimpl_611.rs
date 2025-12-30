// Generated macro for impl_611 (impl)
macro_rules! Depcrate_strings_ffi_strimpl_611 {
() => {
// Module: crate::strings::ffi_str
// Provides: {"impl_611"}
// Dependencies: {}
impl JNIString { # [doc = " Converts a Rust string (in standard UTF-8 encoding) into a"] # [doc = " Java-compatible string (in Java's [modified UTF-8] encoding)."] # [doc = ""] # [doc = " [modified UTF-8]: https://en.wikipedia.org/wiki/UTF-8#Modified_UTF-8"] pub fn new (string : impl AsRef < str >) -> Self { string . into () } # [doc = " Converts a `CString` into a `JNIString`."] # [doc = ""] # [doc = " This method is zero-cost."] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `string` must be in [modified UTF-8] encoding."] # [doc = ""] # [doc = " [modified UTF-8]: https://en.wikipedia.org/wiki/UTF-8#Modified_UTF-8"] pub unsafe fn from_cstring (string : CString) -> Self { Self { internal : string } } # [doc = " Converts a `JNIString` into a `CString`."] # [doc = ""] # [doc = " This method is zero-cost."] pub fn into_cstring (self) -> CString { self . internal } # [doc = " Borrows this `JNIString` as a `&JNIStr`."] # [doc = ""] # [doc = " This is the `JNIString` equivalent to [`CString::as_c_str`]."] # [doc = ""] # [doc = " Note that `&JNIString` also coerces to `&JNIStr`, even without calling"] # [doc = " this method. For example:"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use jni::strings::{JNIStr, JNIString};"] # [doc = " let string: JNIString;"] # [doc = " # string = unimplemented!();"] # [doc = ""] # [doc = " // This works…"] # [doc = " let borrowed: &JNIStr = string.borrowed();"] # [doc = ""] # [doc = " // …and so does this."] # [doc = " let borrowed: &JNIStr = &string;"] # [doc = " ```"] pub fn borrowed (& self) -> & JNIStr { self } }
};
}
