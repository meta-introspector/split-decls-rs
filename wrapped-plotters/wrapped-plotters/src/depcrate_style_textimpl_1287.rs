// Generated macro for impl_1287 (impl)
macro_rules! Depcrate_style_textimpl_1287 {
() => {
// Module: crate::style::text
// Provides: {"impl_1287"}
// Dependencies: {}
impl IntoTextStyle < 'static > for f64 { fn into_text_style < P : HasDimension > (self , _ : & P) -> TextStyle < 'static > { TextStyle :: from ((FontFamily :: SansSerif , self)) } }
};
}
