// Generated macro for impl_1225 (impl)
macro_rules! Depcrate_style_font_font_descimpl_1225 {
() => {
// Module: crate::style::font::font_desc
// Provides: {"impl_1225"}
// Dependencies: {}
impl < 'a , T : Into < f64 > > From < (& 'a str , T) > for FontDesc < 'a > { fn from ((typeface , size) : (& 'a str , T)) -> FontDesc < 'a > { FontDesc :: new (typeface . into () , size . into () , FontStyle :: Normal) } }
};
}
