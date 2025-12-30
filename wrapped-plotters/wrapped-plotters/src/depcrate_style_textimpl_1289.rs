// Generated macro for impl_1289 (impl)
macro_rules! Depcrate_style_textimpl_1289 {
() => {
// Module: crate::style::text
// Provides: {"impl_1289"}
// Dependencies: {}
impl < 'a , F : Into < FontFamily < 'a > > , T : SizeDesc > IntoTextStyle < 'a > for (F , T) { fn into_text_style < P : HasDimension > (self , parent : & P) -> TextStyle < 'a > { (self . 0 . into () , self . 1 . in_pixels (parent)) . into () } }
};
}
