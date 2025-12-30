// Generated macro for impl_13 (impl)
macro_rules! Depcrate_arithmeticimpl_13 {
() => {
// Module: crate::arithmetic
// Provides: {"impl_13"}
// Dependencies: {}
# [doc = " Adapted from [NIST SP 800-186] § G.1.2: Curve P-256."] # [doc = ""] # [doc = " [NIST SP 800-186]: https://csrc.nist.gov/publications/detail/sp/800-186/final"] impl PrimeCurveParams for NistP256 { type FieldElement = FieldElement ; type PointArithmetic = point_arithmetic :: EquationAIsMinusThree ; # [doc = " a = -3"] const EQUATION_A : FieldElement = FieldElement :: from_u64 (3) . neg () ; const EQUATION_B : FieldElement = FieldElement :: from_hex_vartime ("5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b" ,) ; # [doc = " Base point of P-256."] # [doc = ""] # [doc = " Defined in NIST SP 800-186 § G.1.2:"] # [doc = ""] # [doc = " ```text"] # [doc = " Gₓ = 6b17d1f2 e12c4247 f8bce6e5 63a440f2 77037d81 2deb33a0 f4a13945 d898c296"] # [doc = " Gᵧ = 4fe342e2 fe1a7f9b 8ee7eb4a 7c0f9e16 2bce3357 6b315ece cbb64068 37bf51f5"] # [doc = " ```"] const GENERATOR : (FieldElement , FieldElement) = (FieldElement :: from_hex_vartime ("6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296" ,) , FieldElement :: from_hex_vartime ("4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5" ,) ,) ; }
};
}
