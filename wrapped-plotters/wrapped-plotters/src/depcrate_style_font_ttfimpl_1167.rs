// Generated macro for impl_1167 (impl)
macro_rules! Depcrate_style_font_ttfimpl_1167 {
() => {
// Module: crate::style::font::ttf
// Provides: {"impl_1167"}
// Dependencies: {}
impl FontExt { fn new (font : Font) -> Self { let handle = font . handle () ; let (data , idx) = match handle . as_ref () { Some (Handle :: Memory { bytes , font_index }) => (& bytes [..] , * font_index) , _ => unreachable ! () , } ; let face = unsafe { std :: mem :: transmute :: < Option < _ > , Option < Face < 'static > > > (ttf_parser :: Face :: parse (data , idx) . ok () ,) } ; Self { inner : font , face } } fn query_kerning_table (& self , prev : u32 , next : u32) -> f32 { if let Some (face) = self . face . as_ref () { if let Some (kern) = face . tables () . kern { let kern = kern . subtables . into_iter () . filter (| st | st . horizontal && ! st . variable) . filter_map (| st | st . glyphs_kerning (GlyphId (prev as u16) , GlyphId (next as u16))) . next () . unwrap_or (0) ; return kern as f32 ; } } 0.0 } }
};
}
