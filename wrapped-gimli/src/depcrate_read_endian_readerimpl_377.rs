// Generated macro for impl_377 (impl)
macro_rules! Depcrate_read_endian_readerimpl_377 {
() => {
// Module: crate::read::endian_reader
// Provides: {"impl_377"}
// Dependencies: {}
impl < Endian , T > Index < usize > for EndianReader < Endian , T > where Endian : Endianity , T : CloneStableDeref < Target = [u8] > + Debug , { type Output = u8 ; fn index (& self , idx : usize) -> & Self :: Output { & self . bytes () [idx] } }
};
}
