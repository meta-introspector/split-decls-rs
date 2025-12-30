// Generated macro for impl_444 (impl)
macro_rules! Depcrate_trievalueimpl_444 {
() => {
// Module: crate::trievalue
// Provides: {"impl_444"}
// Dependencies: {}
impl TrieValue for BidiMirroringGlyph { type TryFromU32Error = u32 ; fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > { let code_point = i & 0x1FFFFF ; let mirroring_glyph = if code_point == 0 { None } else { Some (char :: try_from_u32 (code_point) . map_err (| _ | i) ?) } ; let mirrored = ((i >> 21) & 0x1) == 1 ; let paired_bracket_type = { let value = ((i >> 22) & 0x3) as u8 ; match value { 0 => crate :: bidi :: BidiPairedBracketType :: None , 1 => crate :: bidi :: BidiPairedBracketType :: Open , 2 => crate :: bidi :: BidiPairedBracketType :: Close , _ => return Err (i) , } } ; Ok (Self { mirrored , mirroring_glyph , paired_bracket_type , }) } fn to_u32 (self) -> u32 { self . mirroring_glyph . unwrap_or_default () as u32 | ((self . mirrored as u32) << 21) | (match self . paired_bracket_type { crate :: bidi :: BidiPairedBracketType :: None => 0 , crate :: bidi :: BidiPairedBracketType :: Open => 1 , crate :: bidi :: BidiPairedBracketType :: Close => 2 , } << 22) } }
};
}
