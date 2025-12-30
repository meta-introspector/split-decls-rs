// Generated macro for impl_28 (impl)
macro_rules! Depcrate_hashimpl_28 {
() => {
// Module: crate::hash
// Provides: {"impl_28"}
// Dependencies: {}
impl FloatHash for f64 { fn fhash (& self) -> i128 { let bits = self . to_bits () ; let sign_bit = bits >> 63 ; let mantissa_bits = bits & 0xfffffffffffff ; let mut exponent : i16 = ((bits >> 52) & 0x7ff) as i16 ; if exponent == 0x7ff { if mantissa_bits != 0 { HASH_NAN } else if sign_bit > 0 { HASH_NEGINF } else { HASH_INF } } else { let mantissa = if exponent == 0 { mantissa_bits << 1 } else { mantissa_bits | 0x10000000000000 } ; exponent -= 0x3ff + 52 ; let mantissa = MInt :: new (mantissa as u128 , & M127U) ; let pow = mantissa . convert (1 << exponent . absm (& 127)) ; let v = mantissa * pow ; v . residue () as i128 * if sign_bit == 0 { 1 } else { - 1 } } } }
};
}
