// Generated macro for impl_597 (impl)
macro_rules! Depcrate_lexical_mathimpl_597 {
() => {
// Module: crate::lexical::math
// Provides: {"impl_597"}
// Dependencies: {}
impl Hi64 < u64 > for [u64] { # [inline] fn hi64_1 (& self) -> (u64 , bool) { debug_assert ! (self . len () == 1) ; let r0 = self [0] ; u64_to_hi64_1 (r0) } # [inline] fn hi64_2 (& self) -> (u64 , bool) { debug_assert ! (self . len () >= 2) ; let r0 = self [self . len () - 1] ; let r1 = self [self . len () - 2] ; let (v , n) = u64_to_hi64_2 (r0 , r1) ; (v , n || nonzero (self , 2)) } # [inline] fn hi64_3 (& self) -> (u64 , bool) { self . hi64_2 () } }
};
}
