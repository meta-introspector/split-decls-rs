// Generated macro for impl_148 (impl)
macro_rules! Depcrate_int_bigimpl_148 {
() => {
// Module: crate::int::big
// Provides: {"impl_148"}
// Dependencies: {}
impl HInt for i128 { type D = i256 ; fn widen (self) -> Self :: D { let mut ret = self . unsigned () . zero_widen () . signed () ; if self . is_negative () { ret . 0 [2] = u64 :: MAX ; ret . 0 [3] = u64 :: MAX ; } ret } fn zero_widen (self) -> Self :: D { self . unsigned () . zero_widen () . signed () } fn zero_widen_mul (self , rhs : Self) -> Self :: D { self . unsigned () . zero_widen_mul (rhs . unsigned ()) . signed () } fn widen_mul (self , rhs : Self) -> Self :: D { unimplemented ! ("signed i128 widening multiply is not used") } fn widen_hi (self) -> Self :: D { self . widen () << < Self as MinInt > :: BITS } }
};
}
