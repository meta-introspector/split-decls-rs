// Generated macro for impl_48 (impl)
macro_rules! Depcrate_provider_dataimpl_48 {
() => {
// Module: crate::provider::data
// Provides: {"impl_48"}
// Dependencies: {}
impl DotType { pub (crate) const DOT_MASK : u16 = 0x3 ; # [inline] pub (crate) fn from_masked_bits (b : u16) -> Self { debug_assert ! (b & Self :: DOT_MASK == b) ; match b { 0 => DotType :: NoDot , 1 => DotType :: SoftDotted , 2 => DotType :: Above , _ => DotType :: OtherAccent , } } }
};
}
