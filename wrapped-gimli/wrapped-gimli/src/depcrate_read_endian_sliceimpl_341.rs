// Generated macro for impl_341 (impl)
macro_rules! Depcrate_read_endian_sliceimpl_341 {
() => {
// Module: crate::read::endian_slice
// Provides: {"impl_341"}
// Dependencies: {}
impl < 'input , Endian > Deref for EndianSlice < 'input , Endian > where Endian : Endianity , { type Target = [u8] ; fn deref (& self) -> & Self :: Target { self . slice } }
};
}
