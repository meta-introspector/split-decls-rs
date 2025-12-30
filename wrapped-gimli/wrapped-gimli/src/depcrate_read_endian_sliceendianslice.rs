// Generated macro for EndianSlice (struct)
macro_rules! Depcrate_read_endian_sliceEndianSlice {
() => {
// Module: crate::read::endian_slice
// Provides: {"EndianSlice"}
// Dependencies: {}
# [doc = " A `&[u8]` slice with endianity metadata."] # [doc = ""] # [doc = " This implements the `Reader` trait, which is used for all reading of DWARF sections."] # [derive (Default , Clone , Copy , PartialEq , Eq , Hash)] pub struct EndianSlice < 'input , Endian > where Endian : Endianity , { slice : & 'input [u8] , endian : Endian , }
};
}
