// Generated macro for impl_223 (impl)
macro_rules! Depcrate_output_captureimpl_223 {
() => {
// Module: crate::output_capture
// Provides: {"impl_223"}
// Dependencies: {}
impl CaptureBuf { pub (crate) fn new () -> Self { Self { inner : Mutex :: new (String :: new ()) } } pub (crate) fn into_inner (self) -> String { self . inner . into_inner () . unwrap_or_else (| e | e . into_inner ()) } }
};
}
