// Generated macro for FontPackageHdr (struct)
macro_rules! Depcrate_hiiFontPackageHdr {
() => {
// Module: crate::hii
// Provides: {"FontPackageHdr"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct FontPackageHdr < const N : usize = 0 > { pub header : PackageHeader , pub hdr_size : u32 , pub glyph_block_offset : u32 , pub cell : GlyphInfo , pub font_style : FontStyle , pub font_family : [crate :: base :: Char16 ; N] , }
};
}
