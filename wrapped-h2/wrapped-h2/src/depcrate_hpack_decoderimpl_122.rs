// Generated macro for impl_122 (impl)
macro_rules! Depcrate_hpack_decoderimpl_122 {
() => {
// Module: crate::hpack::decoder
// Provides: {"impl_122"}
// Dependencies: {}
impl Representation { pub fn load (byte : u8) -> Result < Representation , DecoderError > { const INDEXED : u8 = 0b1000_0000 ; const LITERAL_WITH_INDEXING : u8 = 0b0100_0000 ; const LITERAL_WITHOUT_INDEXING : u8 = 0b1111_0000 ; const LITERAL_NEVER_INDEXED : u8 = 0b0001_0000 ; const SIZE_UPDATE_MASK : u8 = 0b1110_0000 ; const SIZE_UPDATE : u8 = 0b0010_0000 ; if byte & INDEXED == INDEXED { Ok (Representation :: Indexed) } else if byte & LITERAL_WITH_INDEXING == LITERAL_WITH_INDEXING { Ok (Representation :: LiteralWithIndexing) } else if byte & LITERAL_WITHOUT_INDEXING == 0 { Ok (Representation :: LiteralWithoutIndexing) } else if byte & LITERAL_WITHOUT_INDEXING == LITERAL_NEVER_INDEXED { Ok (Representation :: LiteralNeverIndexed) } else if byte & SIZE_UPDATE_MASK == SIZE_UPDATE { Ok (Representation :: SizeUpdate) } else { Err (DecoderError :: InvalidRepresentation) } } }
};
}
