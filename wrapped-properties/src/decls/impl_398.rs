macro_rules! deps {
    () => {
        BidiMirroringGlyph!();
    };
}

macro_rules! impl_398 {
    () => {
        deps!();
        impl AsULE for BidiMirroringGlyph { type ULE = zerovec :: ule :: RawBytesULE < 3 > ; fn to_unaligned (self) -> Self :: ULE { let [a , b , c , _] = TrieValue :: to_u32 (self) . to_le_bytes () ; RawBytesULE ([a , b , c]) } fn from_unaligned (unaligned : Self :: ULE) -> Self { let [a , b , c] = unaligned . 0 ; TrieValue :: try_from_u32 (u32 :: from_le_bytes ([a , b , c , 0])) . unwrap_or_default () } }
    };
}

impl_398!();