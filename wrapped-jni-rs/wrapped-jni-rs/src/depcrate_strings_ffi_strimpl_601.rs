// Generated macro for impl_601 (impl)
macro_rules! Depcrate_strings_ffi_strimpl_601 {
() => {
// Module: crate::strings::ffi_str
// Provides: {"impl_601"}
// Dependencies: {}
impl :: std :: ops :: Deref for JNIString { type Target = JNIStr ; fn deref (& self) -> & Self :: Target { unsafe { JNIStr :: from_ptr (self . internal . as_ptr ()) } } }
};
}
