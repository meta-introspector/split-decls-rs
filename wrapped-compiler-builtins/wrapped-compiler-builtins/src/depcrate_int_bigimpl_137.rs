// Generated macro for impl_137 (impl)
macro_rules! Depcrate_int_bigimpl_137 {
() => {
// Module: crate::int::big
// Provides: {"impl_137"}
// Dependencies: {}
impl u256 { pub const MAX : Self = Self ([u64 :: MAX , u64 :: MAX , u64 :: MAX , u64 :: MAX]) ; # [doc = " Reinterpret as a signed integer"] pub fn signed (self) -> i256 { i256 (self . 0) } }
};
}
