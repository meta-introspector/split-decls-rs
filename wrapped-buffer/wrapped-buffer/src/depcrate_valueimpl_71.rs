// Generated macro for impl_71 (impl)
macro_rules! Depcrate_valueimpl_71 {
() => {
// Module: crate::value
// Provides: {"impl_71"}
// Dependencies: {}
impl Value < 'static > { # [doc = "\n    Fully buffer a value, including any internal borrowed data.\n\n    This method will fail if the `alloc` feature is not enabled.\n    "] pub fn collect_owned (v : impl sval :: Value) -> Result < Self , Error > { ValueBuf :: collect_owned (v) . map (| buf | buf . into_value ()) } }
};
}
