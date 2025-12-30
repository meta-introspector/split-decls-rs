// Generated macro for impl_202 (impl)
macro_rules! Depcrate_math_support_feature_detectimpl_202 {
() => {
// Module: crate::math::support::feature_detect
// Provides: {"impl_202"}
// Dependencies: {}
# [allow (dead_code)] impl Flags { # [doc = " No bits set."] pub const fn empty () -> Self { Self (0) } # [doc = " Create with bits already set."] pub const fn from_bits (val : u32) -> Self { Self (val) } # [doc = " Get the integer representation."] pub fn bits (& self) -> u32 { self . 0 } # [doc = " Set any bits in `mask`."] pub fn insert (& mut self , mask : u32) { self . 0 |= mask ; } # [doc = " Check whether the mask is set."] pub fn contains (& self , mask : u32) -> bool { self . 0 & mask == mask } # [doc = " Check whether the nth bit is set."] pub fn test_nth (& self , bit : u32) -> bool { debug_assert ! (bit < u32 :: BITS , "bit index out-of-bounds") ; self . 0 & (1 << bit) != 0 } }
};
}
