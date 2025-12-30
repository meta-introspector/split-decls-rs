// Generated macro for impl_1226 (impl)
macro_rules! Depcrate_style_font_font_descimpl_1226 {
() => {
// Module: crate::style::font::font_desc
// Provides: {"impl_1226"}
// Dependencies: {}
impl < 'a , T : Into < f64 > , S : Into < FontStyle > > From < (FontFamily < 'a > , T , S) > for FontDesc < 'a > { fn from ((family , size , style) : (FontFamily < 'a > , T , S)) -> FontDesc < 'a > { FontDesc :: new (family , size . into () , style . into ()) } }
};
}
