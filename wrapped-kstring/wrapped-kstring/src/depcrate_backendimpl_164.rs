// Generated macro for impl_164 (impl)
macro_rules! Depcrate_backendimpl_164 {
() => {
// Module: crate::backend
// Provides: {"impl_164"}
// Dependencies: {}
impl HeapStr for BoxedStr { # [inline] fn from_str (other : & str) -> Self { other . into () } # [inline] fn from_string (other : String) -> Self { other . into_boxed_str () } # [inline] fn from_boxed_str (other : BoxedStr) -> Self { other } # [inline] fn as_str (& self) -> & str { self } }
};
}
