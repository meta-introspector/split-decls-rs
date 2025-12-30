// Generated macro for impl_1160 (impl)
macro_rules! Depcrate_style_font_ttfimpl_1160 {
() => {
// Module: crate::style::font::ttf
// Provides: {"impl_1160"}
// Dependencies: {}
impl std :: fmt :: Display for FontError { fn fmt (& self , fmt : & mut std :: fmt :: Formatter) -> Result < () , std :: fmt :: Error > { match self { FontError :: LockError => write ! (fmt , "Could not lock mutex") , FontError :: NoSuchFont (family , style) => { write ! (fmt , "No such font: {} {}" , family , style) } FontError :: FontLoadError (e) => write ! (fmt , "Font loading error {}" , e) , FontError :: GlyphError (e) => write ! (fmt , "Glyph error {}" , e) , } } }
};
}
