// Generated macro for impl_972 (impl)
macro_rules! Depcrate_util_wireimpl_972 {
() => {
// Module: crate::util::wire
// Provides: {"impl_972"}
// Dependencies: {}
impl Endian for BE { fn write_u16 (n : u16 , dst : & mut [u8]) { dst [.. 2] . copy_from_slice (& n . to_be_bytes ()) ; } fn write_u32 (n : u32 , dst : & mut [u8]) { dst [.. 4] . copy_from_slice (& n . to_be_bytes ()) ; } fn write_u128 (n : u128 , dst : & mut [u8]) { dst [.. 16] . copy_from_slice (& n . to_be_bytes ()) ; } }
};
}
