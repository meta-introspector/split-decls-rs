// Generated macro for impl_31 (impl)
macro_rules! Depcrate_streamimpl_31 {
() => {
// Module: crate::stream
// Provides: {"impl_31"}
// Dependencies: {}
impl JsStream { fn next_future (& self) -> Result < JsFuture , JsValue > { self . iter . next () . map (JsFuture :: from) } }
};
}
