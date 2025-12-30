// Generated macro for impl_1227 (impl)
macro_rules! Depcrate_style_font_font_descimpl_1227 {
() => {
// Module: crate::style::font::font_desc
// Provides: {"impl_1227"}
// Dependencies: {}
impl < 'a , T : Into < f64 > , S : Into < FontStyle > > From < (& 'a str , T , S) > for FontDesc < 'a > { fn from ((typeface , size , style) : (& 'a str , T , S)) -> FontDesc < 'a > { FontDesc :: new (typeface . into () , size . into () , style . into ()) } }
};
}
