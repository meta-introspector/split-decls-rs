// Generated macro for JNIStr (struct)
macro_rules! Depcrate_strings_ffi_strJNIStr {
() => {
// Module: crate::strings::ffi_str
// Provides: {"JNIStr"}
// Dependencies: {}
# [doc = " A borrowed null-terminated string (like [`CStr`]) encoded in Java's"] # [doc = " [modified UTF-8]."] # [doc = ""] # [doc = " [`JNIStr`] is to [`JNIString`] as `CStr` is to `CString` and as `str` is to"] # [doc = " `String`."] # [doc = ""] # [doc = " Similar to `CStr` and [`str`], instances of `JNIStr` are borrowed from a"] # [doc = " [`JNIString`]."] # [doc = ""] # [doc = " [JNIStr] is generally used for passing string arguments to JNI functions or"] # [doc = " for viewing the borrowed contents of a `java.lang.String` object."] # [doc = ""] # [doc = " As a special-case, a `&CStr` can be coerced into a `&JNIStr` if the `CStr`"] # [doc = " has a valid modified UTF-8 encoding. (See [`JNIStr::from_cstr`] or"] # [doc = " [`JNIStr::from_cstr_unchecked`])."] # [doc = ""] # [doc = " To convert a `JNIStr` into an ordinary Rust string, use the"] # [doc = " [`to_str`][Self::to_str] method or `to_string`. To get a view of the"] # [doc = " modified UTF-8 encoding of the `JNIStr`, use the [`Self::to_bytes`] method."] # [doc = ""] # [doc = " Note that, as with `CStr`, this type is **not** `repr(C)`. See [the `CStr`"] # [doc = " documentation][CStr] for an explanation of what that means. (This type is"] # [doc = " `repr(transparent)`, but it wraps around a `CStr`, not a raw pointer.)"] # [doc = ""] # [doc = " [modified UTF-8]: https://en.wikipedia.org/wiki/UTF-8#Modified_UTF-8"] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct JNIStr { internal : CStr , }
};
}
