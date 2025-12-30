// Generated macro for impl_613 (impl)
macro_rules! Depcrate_provider_pattern_item_uleimpl_613 {
() => {
// Module: crate::provider::pattern::item::ule
// Provides: {"impl_613"}
// Dependencies: {}
impl GenericPatternItem { # [inline] pub (crate) const fn to_unaligned_const (self) -> < Self as AsULE > :: ULE { match self { Self :: Placeholder (idx) => GenericPatternItemULE ([0b1000_0000 , 0x00 , idx]) , Self :: Literal (ch) => { let u = ch as u32 ; let bytes = u . to_be_bytes () ; GenericPatternItemULE ([bytes [1] , bytes [2] , bytes [3]]) } } } }
};
}
