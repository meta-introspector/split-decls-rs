// Generated macro for GibtVariabilityBlock (struct)
macro_rules! Depcrate_hiiGibtVariabilityBlock {
() => {
// Module: crate::hii
// Provides: {"GibtVariabilityBlock"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct GibtVariabilityBlock < const N : usize = 0 > { pub header : GlyphBlock , pub cell : GlyphInfo , pub glyph_pack_in_bits : u8 , pub bitmap_data : [u8 ; N] , }
};
}
