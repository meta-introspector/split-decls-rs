// Generated macro for impl_368 (impl)
macro_rules! Depcrate_genericimpl_368 {
() => {
// Module: crate::generic
// Provides: {"impl_368"}
// Dependencies: {}
impl Vec4 < u64 > for u64x4_generic { # [inline (always)] fn extract (self , i : u32) -> u64 { let d : [u64 ; 4] = self . to_lanes () ; d [i as usize] } # [inline (always)] fn insert (self , v : u64 , i : u32) -> Self { self . 0 [(i / 2) as usize] . insert (v , i % 2) ; self } }
};
}
