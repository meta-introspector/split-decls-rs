// Generated macro for impl_22 (impl)
macro_rules! Depcrate_algebraimpl_22 {
() => {
// Module: crate::algebra
// Provides: {"impl_22"}
// Dependencies: {}
impl < K : ArraySize > AlgebraExt for Vector < K > { fn mod_plus_minus < M : Unsigned > (& self) -> Self { Self (self . 0 . iter () . map (AlgebraExt :: mod_plus_minus :: < M >) . collect ()) } fn infinity_norm (& self) -> u32 { self . 0 . iter () . map (AlgebraExt :: infinity_norm) . max () . unwrap () } fn power2round (& self) -> (Self , Self) { let mut r1 = Self :: default () ; let mut r0 = Self :: default () ; for (i , x) in self . 0 . iter () . enumerate () { (r1 . 0 [i] , r0 . 0 [i]) = x . power2round () ; } (r1 , r0) } fn high_bits < TwoGamma2 : Unsigned > (& self) -> Self { Self (self . 0 . iter () . map (AlgebraExt :: high_bits :: < TwoGamma2 >) . collect () ,) } fn low_bits < TwoGamma2 : Unsigned > (& self) -> Self { Self (self . 0 . iter () . map (AlgebraExt :: low_bits :: < TwoGamma2 >) . collect () ,) } }
};
}
