// Generated macro for impl_974 (impl)
macro_rules! Depcrate_util_wireimpl_974 {
() => {
// Module: crate::util::wire
// Provides: {"impl_974"}
// Dependencies: {}
impl Endian for LE { fn write_u16 (n : u16 , dst : & mut [u8]) { dst [.. 2] . copy_from_slice (& n . to_le_bytes ()) ; } fn write_u32 (n : u32 , dst : & mut [u8]) { dst [.. 4] . copy_from_slice (& n . to_le_bytes ()) ; } fn write_u128 (n : u128 , dst : & mut [u8]) { dst [.. 16] . copy_from_slice (& n . to_le_bytes ()) ; } }
};
}
