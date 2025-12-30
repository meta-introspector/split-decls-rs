// Generated macro for impl_609 (impl)
macro_rules! Depcrate_provider_pattern_item_uleimpl_609 {
() => {
// Module: crate::provider::pattern::item::ule
// Provides: {"impl_609"}
// Dependencies: {}
impl AsULE for PatternItem { type ULE = PatternItemULE ; # [inline] fn to_unaligned (self) -> Self :: ULE { match self { Self :: Field (field) => { PatternItemULE ([0b1000_0000 , field . symbol . idx () , field . length . idx ()]) } Self :: Literal (ch) => { let u = ch as u32 ; let bytes = u . to_be_bytes () ; PatternItemULE ([bytes [1] , bytes [2] , bytes [3]]) } } } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { let value = unaligned . 0 ; # [expect (clippy :: unwrap_used)] if PatternItemULE :: determine_field_from_u8 (value [0]) { let symbol = fields :: FieldSymbol :: from_idx (value [1]) . unwrap () ; let length = fields :: FieldLength :: from_idx (value [2]) . unwrap () ; PatternItem :: Field (fields :: Field { symbol , length }) } else { PatternItem :: Literal (unsafe { char :: from_u32_unchecked (u32 :: from_be_bytes ([0x00 , value [0] , value [1] , value [2]])) }) } } }
};
}
