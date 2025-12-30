// Generated macro for impl_1224 (impl)
macro_rules! Depcrate_style_font_font_descimpl_1224 {
() => {
// Module: crate::style::font::font_desc
// Provides: {"impl_1224"}
// Dependencies: {}
impl < 'a , T : Into < f64 > > From < (FontFamily < 'a > , T) > for FontDesc < 'a > { fn from ((family , size) : (FontFamily < 'a > , T)) -> FontDesc < 'a > { FontDesc :: new (family , size . into () , FontStyle :: Normal) } }
};
}
