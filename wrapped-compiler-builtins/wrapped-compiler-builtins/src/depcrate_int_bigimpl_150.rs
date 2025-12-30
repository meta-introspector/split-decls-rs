// Generated macro for impl_150 (impl)
macro_rules! Depcrate_int_bigimpl_150 {
() => {
// Module: crate::int::big
// Provides: {"impl_150"}
// Dependencies: {}
impl DInt for i256 { type H = i128 ; fn lo (self) -> Self :: H { let mut tmp = [0u8 ; 16] ; tmp [.. 8] . copy_from_slice (& self . 0 [0] . to_le_bytes ()) ; tmp [8 ..] . copy_from_slice (& self . 0 [1] . to_le_bytes ()) ; i128 :: from_le_bytes (tmp) } fn hi (self) -> Self :: H { let mut tmp = [0u8 ; 16] ; tmp [.. 8] . copy_from_slice (& self . 0 [2] . to_le_bytes ()) ; tmp [8 ..] . copy_from_slice (& self . 0 [3] . to_le_bytes ()) ; i128 :: from_le_bytes (tmp) } }
};
}
