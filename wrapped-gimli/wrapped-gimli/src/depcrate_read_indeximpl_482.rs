// Generated macro for impl_482 (impl)
macro_rules! Depcrate_read_indeximpl_482 {
() => {
// Module: crate::read::index
// Provides: {"impl_482"}
// Dependencies: {}
impl < 'input , Endian > DebugCuIndex < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugCuIndex` instance from the data in the `.debug_cu_index`"] # [doc = " section."] pub fn new (section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (section , endian)) } }
};
}
