// Generated macro for impl_140 (impl)
macro_rules! Depcrate_int_bigimpl_140 {
() => {
// Module: crate::int::big
// Provides: {"impl_140"}
// Dependencies: {}
impl MinInt for u256 { type OtherSign = i256 ; type Unsigned = u256 ; const SIGNED : bool = false ; const BITS : u32 = 256 ; const ZERO : Self = Self ([0u64 ; 4]) ; const ONE : Self = Self ([1 , 0 , 0 , 0]) ; const MIN : Self = Self ([0u64 ; 4]) ; const MAX : Self = Self ([u64 :: MAX ; 4]) ; }
};
}
