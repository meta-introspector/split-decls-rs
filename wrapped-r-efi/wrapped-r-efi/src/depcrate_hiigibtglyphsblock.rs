// Generated macro for GibtGlyphsBlock (struct)
macro_rules! Depcrate_hiiGibtGlyphsBlock {
() => {
// Module: crate::hii
// Provides: {"GibtGlyphsBlock"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct GibtGlyphsBlock < const N : usize = 0 > { pub header : GlyphBlock , pub cell : GlyphInfo , pub count : u16 , pub bitmap_data : [u8 ; N] , }
};
}
