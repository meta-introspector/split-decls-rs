// Generated macro for impl_1280 (impl)
macro_rules! Depcrate_style_textimpl_1280 {
() => {
// Module: crate::style::text
// Provides: {"impl_1280"}
// Dependencies: {}
impl < 'a , T : IntoTextStyle < 'a > > IntoTextStyle < 'a > for TextStyleBuilder < 'a , T > { fn into_text_style < P : HasDimension > (self , parent : & P) -> TextStyle < 'a > { let mut base = self . base . into_text_style (parent) ; if let Some (color) = self . new_color { base . color = color ; } if let Some (pos) = self . new_pos { base = base . pos (pos) ; } base } }
};
}
