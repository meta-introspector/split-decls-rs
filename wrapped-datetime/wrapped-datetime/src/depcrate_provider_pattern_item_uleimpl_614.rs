// Generated macro for impl_614 (impl)
macro_rules! Depcrate_provider_pattern_item_uleimpl_614 {
() => {
// Module: crate::provider::pattern::item::ule
// Provides: {"impl_614"}
// Dependencies: {}
impl AsULE for GenericPatternItem { type ULE = GenericPatternItemULE ; # [inline] fn to_unaligned (self) -> Self :: ULE { self . to_unaligned_const () } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { let value = unaligned . 0 ; if GenericPatternItemULE :: determine_field_from_u8 (value [0]) { Self :: Placeholder (value [2]) } else { # [expect (clippy :: unwrap_used)] Self :: Literal (char :: try_from (u32 :: from_be_bytes ([0x00 , value [0] , value [1] , value [2]])) . unwrap () ,) } } }
};
}
