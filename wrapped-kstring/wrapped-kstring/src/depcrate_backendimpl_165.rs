// Generated macro for impl_165 (impl)
macro_rules! Depcrate_backendimpl_165 {
() => {
// Module: crate::backend
// Provides: {"impl_165"}
// Dependencies: {}
impl HeapStr for ArcStr { # [inline] fn from_str (other : & str) -> Self { other . into () } # [inline] fn from_string (other : String) -> Self { other . into_boxed_str () . into () } # [inline] fn from_boxed_str (other : BoxedStr) -> Self { other . into () } # [inline] fn as_str (& self) -> & str { self } }
};
}
