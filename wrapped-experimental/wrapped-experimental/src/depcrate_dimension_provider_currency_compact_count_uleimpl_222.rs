// Generated macro for impl_222 (impl)
macro_rules! Depcrate_dimension_provider_currency_compact_count_uleimpl_222 {
() => {
// Module: crate::dimension::provider::currency::compact_count_ule
// Provides: {"impl_222"}
// Dependencies: {}
impl AsULE for CompactCount { type ULE = CompactCountULE ; fn to_unaligned (self) -> Self :: ULE { CompactCountULE (match self { CompactCount :: Standard (count) => count as u8 , CompactCount :: AlphaNextToNumber (count) => (count as u8) | 0b1000_0000 , }) } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { let count = match unaligned . 0 & 0b0000_0111 { 0 => PluralCategory :: Zero , 1 => PluralCategory :: One , 2 => PluralCategory :: Two , 3 => PluralCategory :: Few , 4 => PluralCategory :: Many , 5 => PluralCategory :: Other , _ => unreachable ! () , } ; match unaligned . 0 & 0b1000_0000 { 0 => CompactCount :: Standard (count) , 0b1000_0000 => CompactCount :: AlphaNextToNumber (count) , _ => unreachable ! () , } } }
};
}
