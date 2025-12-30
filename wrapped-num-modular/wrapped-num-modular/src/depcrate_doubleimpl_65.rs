// Generated macro for impl_65 (impl)
macro_rules! Depcrate_doubleimpl_65 {
() => {
// Module: crate::double
// Provides: {"impl_65"}
// Dependencies: {}
impl udouble { # [inline] pub const fn leading_zeros (self) -> u32 { if self . hi == 0 { self . lo . leading_zeros () + umax :: BITS } else { self . hi . leading_zeros () } } # [allow (dead_code)] fn div_rem_2by2 (self , other : Self) -> (Self , Self) { let mut n = self ; let mut d = other ; let mut q = Self { lo : 0 , hi : 0 } ; let nbits = (2 * umax :: BITS - n . leading_zeros ()) as u16 ; let dbits = (2 * umax :: BITS - d . leading_zeros ()) as u16 ; assert ! (dbits != 0 , "division by zero") ; if nbits < dbits { return (q , n) ; } let mut shift = nbits - dbits ; d <<= shift ; loop { if n >= d { q += 1 ; n -= d ; } if shift == 0 { break ; } d >>= 1u8 ; q <<= 1u8 ; shift -= 1 ; } (q , n) } fn div_rem_2by1 (self , other : umax) -> (umax , umax) { const B : umax = 1 << HALF_BITS ; let s = other . leading_zeros () ; let (n , d) = (self << s , other << s) ; let (d1 , d0) = split (d) ; let (n1 , n0) = split (n . lo) ; let (mut q1 , mut rhat) = div_rem (n . hi , d1) ; while q1 >= B || q1 * d0 > B * rhat + n1 { q1 -= 1 ; rhat += d1 ; if rhat >= B { break ; } } let r21 = n . hi . wrapping_mul (B) . wrapping_add (n1) . wrapping_sub (q1 . wrapping_mul (d)) ; let (mut q0 , mut rhat) = div_rem (r21 , d1) ; while q0 >= B || q0 * d0 > B * rhat + n0 { q0 -= 1 ; rhat += d1 ; if rhat >= B { break ; } } let r = (r21 . wrapping_mul (B) . wrapping_add (n0) . wrapping_sub (q0 . wrapping_mul (d))) >> s ; let q = q1 * B + q0 ; (q , r) } }
};
}
