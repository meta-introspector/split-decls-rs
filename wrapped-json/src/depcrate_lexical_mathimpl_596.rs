// Generated macro for impl_596 (impl)
macro_rules! Depcrate_lexical_mathimpl_596 {
() => {
// Module: crate::lexical::math
// Provides: {"impl_596"}
// Dependencies: {}
impl Hi64 < u32 > for [u32] { # [inline] fn hi64_1 (& self) -> (u64 , bool) { debug_assert ! (self . len () == 1) ; let r0 = self [0] as u64 ; u64_to_hi64_1 (r0) } # [inline] fn hi64_2 (& self) -> (u64 , bool) { debug_assert ! (self . len () == 2) ; let r0 = (self [1] as u64) << 32 ; let r1 = self [0] as u64 ; u64_to_hi64_1 (r0 | r1) } # [inline] fn hi64_3 (& self) -> (u64 , bool) { debug_assert ! (self . len () >= 3) ; let r0 = self [self . len () - 1] as u64 ; let r1 = (self [self . len () - 2] as u64) << 32 ; let r2 = self [self . len () - 3] as u64 ; let (v , n) = u64_to_hi64_2 (r0 , r1 | r2) ; (v , n || nonzero (self , 3)) } }
};
}
