// Generated macro for impl_602 (impl)
macro_rules! Depcrate_strings_ffi_strimpl_602 {
() => {
// Module: crate::strings::ffi_str
// Provides: {"impl_602"}
// Dependencies: {}
# [doc = " Converts a `CStr` into a `JNIStr`."] # [doc = ""] # [doc = " # Panic"] # [doc = ""] # [doc = " This function will panic if the `CStr` is not valid modified UTF-8."] impl AsRef < JNIStr > for CStr { fn as_ref (& self) -> & JNIStr { JNIStr :: from_cstr (self) } }
};
}
