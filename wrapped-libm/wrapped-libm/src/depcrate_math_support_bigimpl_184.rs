// Generated macro for impl_184 (impl)
macro_rules! Depcrate_math_support_bigimpl_184 {
() => {
// Module: crate::math::support::big
// Provides: {"impl_184"}
// Dependencies: {}
impl HInt for u128 { type D = u256 ; fn widen (self) -> Self :: D { u256 { lo : self , hi : 0 } } fn zero_widen (self) -> Self :: D { self . widen () } fn zero_widen_mul (self , rhs : Self) -> Self :: D { let l0 = self & U128_LO_MASK ; let l1 = rhs & U128_LO_MASK ; let h0 = self >> 64 ; let h1 = rhs >> 64 ; let p_ll : u128 = l0 . overflowing_mul (l1) . 0 ; let p_lh : u128 = l0 . overflowing_mul (h1) . 0 ; let p_hl : u128 = h0 . overflowing_mul (l1) . 0 ; let p_hh : u128 = h0 . overflowing_mul (h1) . 0 ; let s0 = p_hl + (p_ll >> 64) ; let s1 = (p_ll & U128_LO_MASK) + (s0 << 64) ; let s2 = p_lh + (s1 >> 64) ; let lo = (p_ll & U128_LO_MASK) + (s2 << 64) ; let hi = p_hh + (s0 >> 64) + (s2 >> 64) ; u256 { lo , hi } } fn widen_mul (self , rhs : Self) -> Self :: D { self . zero_widen_mul (rhs) } fn widen_hi (self) -> Self :: D { u256 { lo : 0 , hi : self } } }
};
}
