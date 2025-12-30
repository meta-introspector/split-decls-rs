// Generated macro for impl_20 (impl)
macro_rules! Depcrate_algebraimpl_20 {
() => {
// Module: crate::algebra
// Provides: {"impl_20"}
// Dependencies: {}
impl AlgebraExt for Elem { fn mod_plus_minus < M : Unsigned > (& self) -> Self { let raw_mod = Elem :: new (M :: reduce (self . 0)) ; if raw_mod . 0 <= M :: U32 >> 1 { raw_mod } else { raw_mod - Elem :: new (M :: U32) } } fn infinity_norm (& self) -> u32 { if self . 0 <= BaseField :: Q >> 1 { self . 0 } else { BaseField :: Q - self . 0 } } fn power2round (& self) -> (Self , Self) { type D = U13 ; type Pow2D = Shleft < U1 , D > ; let r_plus = self . clone () ; let r0 = r_plus . mod_plus_minus :: < Pow2D > () ; let r1 = Elem :: new ((r_plus - r0) . 0 >> D :: USIZE) ; (r1 , r0) } fn high_bits < TwoGamma2 : Unsigned > (& self) -> Self { self . decompose :: < TwoGamma2 > () . 0 } fn low_bits < TwoGamma2 : Unsigned > (& self) -> Self { self . decompose :: < TwoGamma2 > () . 1 } }
};
}
