// Generated macro for impl_254 (impl)
macro_rules! Depcrate_lineimpl_254 {
() => {
// Module: crate::line
// Provides: {"impl_254"}
// Dependencies: {}
impl CTLine { pub fn new_with_attributed_string (string : CFAttributedStringRef) -> Self { unsafe { let ptr = CTLineCreateWithAttributedString (string) ; CTLine :: wrap_under_create_rule (ptr) } } pub fn glyph_runs (& self) -> CFArray < CTRun > { unsafe { TCFType :: wrap_under_get_rule (CTLineGetGlyphRuns (self . 0)) } } pub fn get_string_range (& self) -> CFRange { unsafe { CTLineGetStringRange (self . as_concrete_TypeRef ()) } } pub fn draw (& self , context : & CGContext) { unsafe { CTLineDraw (self . as_concrete_TypeRef () , context . as_ptr ()) } } pub fn get_image_bounds (& self , context : & CGContext) -> CGRect { unsafe { CTLineGetImageBounds (self . as_concrete_TypeRef () , context . as_ptr ()) } } pub fn get_typographic_bounds (& self) -> TypographicBounds { let mut ascent = 0.0 ; let mut descent = 0.0 ; let mut leading = 0.0 ; unsafe { let width = CTLineGetTypographicBounds (self . as_concrete_TypeRef () , & mut ascent , & mut descent , & mut leading ,) ; TypographicBounds { width , ascent , descent , leading , } } } pub fn get_string_index_for_position (& self , position : CGPoint) -> CFIndex { unsafe { CTLineGetStringIndexForPosition (self . as_concrete_TypeRef () , position) } } pub fn get_string_offset_for_string_index (& self , charIndex : CFIndex) -> CGFloat { unsafe { CTLineGetOffsetForStringIndex (self . as_concrete_TypeRef () , charIndex , std :: ptr :: null ()) } } }
};
}
