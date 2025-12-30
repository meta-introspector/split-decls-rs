// Generated macro for fuzz_float_step (function)
macro_rules! Depcratefuzz_float_step {
() => {
// Module: crate
// Provides: {"fuzz_float_step"}
// Dependencies: {}
fn fuzz_float_step < F : Float > (rng : & mut Xoshiro128StarStar , f : & mut F) { let rng32 = rng . next_u32 () ; let sign = (rng32 & 1) != 0 ; let ones = (F :: Int :: ONE << F :: EXP_BITS) - F :: Int :: ONE ; let r0 = (rng32 >> 1) % F :: EXP_BITS ; let r1 = (rng32 >> 5) % F :: EXP_BITS ; let mask = if r1 == 0 { ones . wrapping_shr (r0) } else { let tmp = ones . wrapping_shr (r0) ; (tmp . wrapping_shl (r1) | tmp . wrapping_shr (F :: EXP_BITS - r1)) & ones } ; let mut exp = (f . to_bits () & F :: EXP_MASK) >> F :: SIG_BITS ; match (rng32 >> 9) % 4 { 0 => exp |= mask , 1 => exp &= mask , _ => exp ^= mask , } let mut sig = f . to_bits () & F :: SIG_MASK ; fuzz_step (rng , & mut sig) ; sig &= F :: SIG_MASK ; * f = F :: from_parts (sign , exp , sig) ; }
};
}
