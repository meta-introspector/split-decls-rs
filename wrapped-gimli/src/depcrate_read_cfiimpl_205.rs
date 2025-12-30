// Generated macro for impl_205 (impl)
macro_rules! Depcrate_read_cfiimpl_205 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_205"}
// Dependencies: {}
impl < 'input , Endian > EhFrameHdr < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Constructs a new `EhFrameHdr` instance from the data in the `.eh_frame_hdr` section."] pub fn new (section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (section , endian)) } }
};
}
