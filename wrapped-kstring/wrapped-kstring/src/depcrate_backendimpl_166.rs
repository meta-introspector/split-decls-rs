// Generated macro for impl_166 (impl)
macro_rules! Depcrate_backendimpl_166 {
() => {
// Module: crate::backend
// Provides: {"impl_166"}
// Dependencies: {}
impl HeapStr for RcStr { # [inline] fn from_str (other : & str) -> Self { other . into () } # [inline] fn from_string (other : String) -> Self { other . into_boxed_str () . into () } # [inline] fn from_boxed_str (other : BoxedStr) -> Self { other . into () } # [inline] fn as_str (& self) -> & str { self } }
};
}
