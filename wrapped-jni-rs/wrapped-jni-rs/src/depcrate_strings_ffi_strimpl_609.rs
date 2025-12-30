// Generated macro for impl_609 (impl)
macro_rules! Depcrate_strings_ffi_strimpl_609 {
() => {
// Module: crate::strings::ffi_str
// Provides: {"impl_609"}
// Dependencies: {}
impl From < JNIString > for String { fn from (other : JNIString) -> String { other . to_str () . into_owned () } }
};
}
