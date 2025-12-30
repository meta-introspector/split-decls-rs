// Generated macro for GibtGlyphBlock (struct)
macro_rules! Depcrate_hiiGibtGlyphBlock {
() => {
// Module: crate::hii
// Provides: {"GibtGlyphBlock"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct GibtGlyphBlock < const N : usize = 0 > { pub header : GlyphBlock , pub cell : GlyphInfo , pub bitmap_data : [u8 ; N] , }
};
}
