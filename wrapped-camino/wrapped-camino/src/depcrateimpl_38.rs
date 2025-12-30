// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl Clone for Box < Utf8Path > { fn clone (& self) -> Self { let boxed : Box < Path > = self . 0 . into () ; let ptr = Box :: into_raw (boxed) as * mut Utf8Path ; unsafe { Box :: from_raw (ptr) } } }
};
}
