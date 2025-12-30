// Generated macro for impl_1286 (impl)
macro_rules! Depcrate_style_textimpl_1286 {
() => {
// Module: crate::style::text
// Provides: {"impl_1286"}
// Dependencies: {}
impl IntoTextStyle < 'static > for u32 { fn into_text_style < P : HasDimension > (self , _ : & P) -> TextStyle < 'static > { TextStyle :: from ((FontFamily :: SansSerif , self)) } }
};
}
