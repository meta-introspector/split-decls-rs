// Generated macro for integer_decode_f32 (function)
macro_rules! Depcrate_floatinteger_decode_f32 {
() => {
// Module: crate::float
// Provides: {"integer_decode_f32"}
// Dependencies: {}
fn integer_decode_f32 (f : f32) -> (u64 , i16 , i8) { let bits : u32 = f . to_bits () ; let sign : i8 = if bits >> 31 == 0 { 1 } else { - 1 } ; let mut exponent : i16 = ((bits >> 23) & 0xff) as i16 ; let mantissa = if exponent == 0 { (bits & 0x7fffff) << 1 } else { (bits & 0x7fffff) | 0x800000 } ; exponent -= 127 + 23 ; (mantissa as u64 , exponent , sign) }
};
}
