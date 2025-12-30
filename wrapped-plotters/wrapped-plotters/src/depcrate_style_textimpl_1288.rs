// Generated macro for impl_1288 (impl)
macro_rules! Depcrate_style_textimpl_1288 {
() => {
// Module: crate::style::text
// Provides: {"impl_1288"}
// Dependencies: {}
impl < 'a , T : Color > IntoTextStyle < 'a > for & 'a T { fn into_text_style < P : HasDimension > (self , _ : & P) -> TextStyle < 'a > { TextStyle :: from (FontFamily :: SansSerif) . color (self) } }
};
}
