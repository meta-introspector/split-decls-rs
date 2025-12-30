// Generated macro for sqrt (function)
macro_rules! Depcrate_mathsqrt {
() => {
// Module: crate::math
// Provides: {"sqrt"}
// Dependencies: {}
pub (crate) fn sqrt < S : rustc_apfloat :: ieee :: Semantics > (x : IeeeFloat < S >) -> IeeeFloat < S > { match x . category () { rustc_apfloat :: Category :: Zero => x , rustc_apfloat :: Category :: NaN => x , _ if x . is_negative () => IeeeFloat :: NAN , rustc_apfloat :: Category :: Infinity => IeeeFloat :: INFINITY , rustc_apfloat :: Category :: Normal => { let prec = i32 :: try_from (S :: PRECISION) . unwrap () - 1 ; let mut exp = x . ilogb () ; let mut mant = x . scalbn (prec - exp) . to_u128 (128) . value ; if exp % 2 != 0 { exp -= 1 ; mant <<= 1 ; } let mut res = 0u128 ; let mut rem = mant << 1 ; let mut s = 0u128 ; let mut d = 1u128 << (prec + 1) ; while d != 0 { let t = s + d ; if rem >= t { res += d ; s += d + d ; rem -= t ; } rem <<= 1 ; d >>= 1 ; } res = (res + 1) >> 1 ; IeeeFloat :: from_u128 (res) . value . scalbn (exp / 2 - prec) } } }
};
}
