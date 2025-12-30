// Generated macro for impl_1290 (impl)
macro_rules! Depcrate_style_textimpl_1290 {
() => {
// Module: crate::style::text
// Provides: {"impl_1290"}
// Dependencies: {}
impl < 'a , F : Into < FontFamily < 'a > > , T : SizeDesc , C : Color > IntoTextStyle < 'a > for (F , T , & 'a C) { fn into_text_style < P : HasDimension > (self , parent : & P) -> TextStyle < 'a > { IntoTextStyle :: into_text_style ((self . 0 , self . 1) , parent) . color (self . 2) } }
};
}
