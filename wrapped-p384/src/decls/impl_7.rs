macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        # [doc = " Adapted from [NIST SP 800-186] § G.1.3: Curve P-384."] # [doc = ""] # [doc = " [NIST SP 800-186]: https://csrc.nist.gov/publications/detail/sp/800-186/final"] impl PrimeCurveParams for NistP384 { type FieldElement = FieldElement ; type PointArithmetic = point_arithmetic :: EquationAIsMinusThree ; # [doc = " a = -3 (0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000fffffffc)"] const EQUATION_A : FieldElement = FieldElement :: from_u64 (3) . neg () ; # [doc = " b = b3312fa7 e23ee7e4 988e056b e3f82d19 181d9c6e fe814112"] # [doc = "     0314088f 5013875a c656398d 8a2ed19d 2a85c8ed d3ec2aef"] const EQUATION_B : FieldElement = FieldElement :: from_hex_vartime ("b3312fa7e23ee7e4988e056be3f82d19181d9c6efe8141120314088f5013875ac656398d8a2ed19d2a85c8edd3ec2aef" ,) ; # [doc = " Base point of P-384."] # [doc = ""] # [doc = " Defined in NIST SP 800-186 § G.1.3: Curve P-384."] # [doc = ""] # [doc = " ```text"] # [doc = " Gₓ = aa87ca22 be8b0537 8eb1c71e f320ad74 6e1d3b62 8ba79b98"] # [doc = "      59f741e0 82542a38 5502f25d bf55296c 3a545e38 72760ab7"] # [doc = " Gᵧ = 3617de4a 96262c6f 5d9e98bf 9292dc29 f8f41dbd 289a147c"] # [doc = "      e9da3113 b5f0b8c0 0a60b1ce 1d7e819d 7a431d7c 90ea0e5f"] # [doc = " ```"] const GENERATOR : (FieldElement , FieldElement) = (FieldElement :: from_hex_vartime ("aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab7" ,) , FieldElement :: from_hex_vartime ("3617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f" ,) ,) ; }
    };
}

impl_7!();