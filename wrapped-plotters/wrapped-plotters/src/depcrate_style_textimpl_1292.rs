// Generated macro for impl_1292 (impl)
macro_rules! Depcrate_style_textimpl_1292 {
() => {
// Module: crate::style::text
// Provides: {"impl_1292"}
// Dependencies: {}
impl < 'a , F : Into < FontFamily < 'a > > , T : SizeDesc , C : Color > IntoTextStyle < 'a > for (F , T , FontStyle , & 'a C) { fn into_text_style < P : HasDimension > (self , parent : & P) -> TextStyle < 'a > { IntoTextStyle :: into_text_style ((self . 0 , self . 1 , self . 2) , parent) . color (self . 3) } }
};
}
