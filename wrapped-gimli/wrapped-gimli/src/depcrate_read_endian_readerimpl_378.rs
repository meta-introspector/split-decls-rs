// Generated macro for impl_378 (impl)
macro_rules! Depcrate_read_endian_readerimpl_378 {
() => {
// Module: crate::read::endian_reader
// Provides: {"impl_378"}
// Dependencies: {}
impl < Endian , T > Index < RangeFrom < usize > > for EndianReader < Endian , T > where Endian : Endianity , T : CloneStableDeref < Target = [u8] > + Debug , { type Output = [u8] ; fn index (& self , idx : RangeFrom < usize >) -> & Self :: Output { & self . bytes () [idx] } }
};
}
