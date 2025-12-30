// Generated macro for impl_610 (impl)
macro_rules! Depcrate_strings_ffi_strimpl_610 {
() => {
// Module: crate::strings::ffi_str
// Provides: {"impl_610"}
// Dependencies: {}
impl std :: fmt :: Display for JNIStr { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let s = self . to_str () ; write ! (f , "{}" , s) } }
};
}
