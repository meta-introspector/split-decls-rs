// Generated macro for impl_149 (impl)
macro_rules! Depcrate_int_bigimpl_149 {
() => {
// Module: crate::int::big
// Provides: {"impl_149"}
// Dependencies: {}
impl DInt for u256 { type H = u128 ; fn lo (self) -> Self :: H { let mut tmp = [0u8 ; 16] ; tmp [.. 8] . copy_from_slice (& self . 0 [0] . to_le_bytes ()) ; tmp [8 ..] . copy_from_slice (& self . 0 [1] . to_le_bytes ()) ; u128 :: from_le_bytes (tmp) } fn hi (self) -> Self :: H { let mut tmp = [0u8 ; 16] ; tmp [.. 8] . copy_from_slice (& self . 0 [2] . to_le_bytes ()) ; tmp [8 ..] . copy_from_slice (& self . 0 [3] . to_le_bytes ()) ; u128 :: from_le_bytes (tmp) } }
};
}
