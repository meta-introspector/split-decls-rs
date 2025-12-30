// Generated macro for impl_608 (impl)
macro_rules! Depcrate_strings_ffi_strimpl_608 {
() => {
// Module: crate::strings::ffi_str
// Provides: {"impl_608"}
// Dependencies: {}
impl < 'str_ref > From < & 'str_ref JNIString > for Cow < 'str_ref , JNIStr > { # [doc = " Converts `&JNIString` into `Cow::<&JNIStr>::Borrowed`. Zero-cost."] fn from (string : & 'str_ref JNIString) -> Self { Cow :: Borrowed (string) } }
};
}
