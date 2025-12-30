// Generated macro for ZETA_POW_BITREV (const)
macro_rules! Depcrate_nttZETA_POW_BITREV {
() => {
// Module: crate::ntt
// Provides: {"ZETA_POW_BITREV"}
// Dependencies: {}
# [allow (clippy :: cast_possible_truncation)] # [allow (clippy :: as_conversions)] # [allow (clippy :: integer_division_remainder_used)] const ZETA_POW_BITREV : [Elem ; 256] = { const ZETA : u64 = 1753 ; const fn bitrev8 (x : usize) -> usize { (x as u8) . reverse_bits () as usize } let mut pow = [Elem :: new (0) ; 256] ; let mut i = 0 ; let mut curr = 1u64 ; while i < 256 { pow [i] = Elem :: new (curr as u32) ; i += 1 ; curr = (curr * ZETA) % BaseField :: QL ; } let mut pow_bitrev = [Elem :: new (0) ; 256] ; let mut i = 1 ; while i < 256 { pow_bitrev [i] = pow [bitrev8 (i)] ; i += 1 ; } pow_bitrev } ;
};
}
