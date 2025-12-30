// Generated macro for impl_12 (impl)
macro_rules! Depcrate_arithmeticimpl_12 {
() => {
// Module: crate::arithmetic
// Provides: {"impl_12"}
// Dependencies: {}
# [doc = " Adapted from [FIPS 186-4] § D.1.2.1: Curve P-192."] # [doc = ""] # [doc = " [FIPS 186-4]: https://csrc.nist.gov/publications/detail/fips/186/4/final"] impl PrimeCurveParams for NistP192 { type FieldElement = FieldElement ; type PointArithmetic = point_arithmetic :: EquationAIsMinusThree ; # [doc = " a = -3 (=0xffffffff ffffffff ffffffff fffffffe ffffffff ffffffff fffffffe)"] const EQUATION_A : FieldElement = FieldElement :: from_u64 (3) . neg () ; # [doc = " b = 0x64210519 e59c80e7 0fa7e9ab 72243049 feb8deec c146b9b1"] const EQUATION_B : FieldElement = FieldElement :: from_hex_vartime ("64210519e59c80e70fa7e9ab72243049feb8deecc146b9b1") ; # [doc = " Base point of P-192."] # [doc = ""] # [doc = " ```text"] # [doc = " Gₓ = 0x188da80e b03090f6 7cbf20eb 43a18800 f4ff0afd 82ff1012"] # [doc = " Gᵧ = 0x07192b95 ffc8da78 631011ed 6b24cdd5 73f977a1 1e794811"] # [doc = " ```"] const GENERATOR : (FieldElement , FieldElement) = (FieldElement :: from_hex_vartime ("188da80eb03090f67cbf20eb43a18800f4ff0afd82ff1012") , FieldElement :: from_hex_vartime ("07192b95ffc8da78631011ed6b24cdd573f977a11e794811") ,) ; }
};
}
