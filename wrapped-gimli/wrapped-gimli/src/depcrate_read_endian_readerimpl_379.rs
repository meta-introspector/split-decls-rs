// Generated macro for impl_379 (impl)
macro_rules! Depcrate_read_endian_readerimpl_379 {
() => {
// Module: crate::read::endian_reader
// Provides: {"impl_379"}
// Dependencies: {}
impl < Endian , T > Deref for EndianReader < Endian , T > where Endian : Endianity , T : CloneStableDeref < Target = [u8] > + Debug , { type Target = [u8] ; fn deref (& self) -> & Self :: Target { self . bytes () } }
};
}
