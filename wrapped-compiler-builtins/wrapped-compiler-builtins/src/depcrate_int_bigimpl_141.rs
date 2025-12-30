// Generated macro for impl_141 (impl)
macro_rules! Depcrate_int_bigimpl_141 {
() => {
// Module: crate::int::big
// Provides: {"impl_141"}
// Dependencies: {}
impl MinInt for i256 { type OtherSign = u256 ; type Unsigned = u256 ; const SIGNED : bool = false ; const BITS : u32 = 256 ; const ZERO : Self = Self ([0u64 ; 4]) ; const ONE : Self = Self ([1 , 0 , 0 , 0]) ; const MIN : Self = Self ([0 , 0 , 0 , 1 << 63]) ; const MAX : Self = Self ([u64 :: MAX , u64 :: MAX , u64 :: MAX , u64 :: MAX >> 1]) ; }
};
}
