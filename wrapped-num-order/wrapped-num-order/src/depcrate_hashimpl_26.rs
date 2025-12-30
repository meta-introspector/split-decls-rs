// Generated macro for impl_26 (impl)
macro_rules! Depcrate_hashimpl_26 {
() => {
// Module: crate::hash
// Provides: {"impl_26"}
// Dependencies: {}
impl FloatHash for f32 { fn fhash (& self) -> i128 { let bits = self . to_bits () ; let sign_bit = bits >> 31 ; let mantissa_bits = bits & 0x7fffff ; let mut exponent : i16 = ((bits >> 23) & 0xff) as i16 ; if exponent == 0xff { if mantissa_bits != 0 { HASH_NAN } else if sign_bit > 0 { HASH_NEGINF } else { HASH_INF } } else { let mantissa = if exponent == 0 { mantissa_bits << 1 } else { mantissa_bits | 0x800000 } ; exponent -= 0x7f + 23 ; let mantissa = MInt :: new (mantissa as u128 , & M127U) ; let pow = mantissa . convert (1 << exponent . absm (& 127)) ; let v = mantissa * pow ; v . residue () as i128 * if sign_bit == 0 { 1 } else { - 1 } } } }
};
}
