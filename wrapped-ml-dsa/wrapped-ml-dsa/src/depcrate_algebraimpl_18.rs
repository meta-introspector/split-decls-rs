// Generated macro for impl_18 (impl)
macro_rules! Depcrate_algebraimpl_18 {
() => {
// Module: crate::algebra
// Provides: {"impl_18"}
// Dependencies: {}
impl Decompose for Elem { fn decompose < TwoGamma2 : Unsigned > (self) -> (Elem , Elem) { let r_plus = self . clone () ; let r0 = r_plus . mod_plus_minus :: < TwoGamma2 > () ; if r_plus - r0 == Elem :: new (BaseField :: Q - 1) { (Elem :: new (0) , r0 - Elem :: new (1)) } else { let mut r1 = r_plus - r0 ; r1 . 0 /= TwoGamma2 :: U32 ; (r1 , r0) } } }
};
}
