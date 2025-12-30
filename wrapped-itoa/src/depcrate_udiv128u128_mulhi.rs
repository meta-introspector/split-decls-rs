// Generated macro for u128_mulhi (function)
macro_rules! Depcrate_udiv128u128_mulhi {
() => {
// Module: crate::udiv128
// Provides: {"u128_mulhi"}
// Dependencies: {}
# [doc = " Multiply unsigned 128 bit integers, return upper 128 bits of the result"] # [inline] # [cfg_attr (feature = "no-panic" , no_panic)] fn u128_mulhi (x : u128 , y : u128) -> u128 { let x_lo = x as u64 ; let x_hi = (x >> 64) as u64 ; let y_lo = y as u64 ; let y_hi = (y >> 64) as u64 ; let carry = (x_lo as u128 * y_lo as u128) >> 64 ; let m = x_lo as u128 * y_hi as u128 + carry ; let high1 = m >> 64 ; let m_lo = m as u64 ; let high2 = (x_hi as u128 * y_lo as u128 + m_lo as u128) >> 64 ; x_hi as u128 * y_hi as u128 + high1 + high2 }
};
}
