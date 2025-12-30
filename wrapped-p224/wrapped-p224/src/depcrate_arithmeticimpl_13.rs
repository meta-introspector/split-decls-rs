// Generated macro for impl_13 (impl)
macro_rules! Depcrate_arithmeticimpl_13 {
() => {
// Module: crate::arithmetic
// Provides: {"impl_13"}
// Dependencies: {}
# [doc = " Adapted from [NIST SP 800-186] § 3.2.1.2: P-224."] # [doc = ""] # [doc = " [NIST SP 800-186]: https://csrc.nist.gov/publications/detail/sp/800-186/final"] impl PrimeCurveParams for NistP224 { type FieldElement = FieldElement ; type PointArithmetic = point_arithmetic :: EquationAIsMinusThree ; # [doc = " a = -3 (=0xffffffff ffffffff ffffffff fffffffe ffffffff ffffffff fffffffe)"] const EQUATION_A : FieldElement = FieldElement :: from_u64 (3) . neg () ; # [doc = " b = 0xb4050a85 0c04b3ab f5413256 5044b0b7 d7bfd8ba 270b3943 2355ffb4"] const EQUATION_B : FieldElement = FieldElement :: from_hex_vartime ("b4050a850c04b3abf54132565044b0b7d7bfd8ba270b39432355ffb4") ; # [doc = " Base point of P-224."] # [doc = ""] # [doc = " ```text"] # [doc = " Gₓ = 0xb70e0cbd 6bb4bf7f 321390b9 4a03c1d3 56c21122 343280d6 115c1d21"] # [doc = " Gᵧ = 0xbd376388 b5f723fb 4c22dfe6 cd4375a0 5a074764 44d58199 85007e34"] # [doc = " ```"] const GENERATOR : (FieldElement , FieldElement) = (FieldElement :: from_hex_vartime ("b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21") , FieldElement :: from_hex_vartime ("bd376388b5f723fb4c22dfe6cd4375a05a07476444d5819985007e34") ,) ; }
};
}
