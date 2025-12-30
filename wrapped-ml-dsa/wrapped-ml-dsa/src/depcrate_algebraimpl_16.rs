// Generated macro for impl_16 (impl)
macro_rules! Depcrate_algebraimpl_16 {
() => {
// Module: crate::algebra
// Provides: {"impl_16"}
// Dependencies: {}
impl < M > BarrettReduce for M where M : Unsigned , { # [allow (clippy :: as_conversions)] const SHIFT : usize = 2 * (M :: U64 . ilog2 () + 1) as usize ; # [allow (clippy :: integer_division_remainder_used)] const MULTIPLIER : u64 = (1 << Self :: SHIFT) / M :: U64 ; }
};
}
