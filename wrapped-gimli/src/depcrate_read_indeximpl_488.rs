// Generated macro for impl_488 (impl)
macro_rules! Depcrate_read_indeximpl_488 {
() => {
// Module: crate::read::index
// Provides: {"impl_488"}
// Dependencies: {}
impl < 'input , Endian > DebugTuIndex < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugTuIndex` instance from the data in the `.debug_tu_index`"] # [doc = " section."] pub fn new (section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (section , endian)) } }
};
}
