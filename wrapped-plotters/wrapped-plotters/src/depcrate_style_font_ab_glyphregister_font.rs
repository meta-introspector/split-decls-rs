// Generated macro for register_font (function)
macro_rules! Depcrate_style_font_ab_glyphregister_font {
() => {
// Module: crate::style::font::ab_glyph
// Provides: {"register_font"}
// Dependencies: {}
# [doc = " Register a font in the fonts table."] # [doc = ""] # [doc = " The `name` parameter gives the name this font shall be referred to"] # [doc = " in the other APIs, like `\"sans-serif\"`."] # [doc = ""] # [doc = " Unprovided font styles for a given name will fallback to `FontStyle::Normal`"] # [doc = " if that is available for that name, when other functions lookup fonts which"] # [doc = " are registered with this function."] # [doc = ""] # [doc = " The `bytes` parameter should be the complete contents"] # [doc = " of an OpenType font file, like:"] # [doc = " ```ignore"] # [doc = " include_bytes!(\"FiraGO-Regular.otf\")"] # [doc = " ```"] pub fn register_font (name : & str , style : FontStyle , bytes : & 'static [u8] ,) -> Result < () , InvalidFont > { let font = FontRef :: try_from_slice (bytes) . map_err (| _ | InvalidFont { _priv : () }) ? ; let mut lock = FONTS . write () . unwrap () ; lock . entry (name . to_string ()) . or_insert_with (FontMap :: new) . insert (style , font) ; Ok (()) }
};
}
