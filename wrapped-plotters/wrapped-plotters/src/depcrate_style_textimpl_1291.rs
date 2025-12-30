// Generated macro for impl_1291 (impl)
macro_rules! Depcrate_style_textimpl_1291 {
() => {
// Module: crate::style::text
// Provides: {"impl_1291"}
// Dependencies: {}
impl < 'a , F : Into < FontFamily < 'a > > , T : SizeDesc > IntoTextStyle < 'a > for (F , T , FontStyle) { fn into_text_style < P : HasDimension > (self , parent : & P) -> TextStyle < 'a > { (self . 0 . into () , self . 1 . in_pixels (parent) , self . 2) . into () } }
};
}
