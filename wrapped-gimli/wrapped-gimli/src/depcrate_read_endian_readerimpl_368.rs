// Generated macro for impl_368 (impl)
macro_rules! Depcrate_read_endian_readerimpl_368 {
() => {
// Module: crate::read::endian_reader
// Provides: {"impl_368"}
// Dependencies: {}
impl < Endian , T1 , T2 > PartialEq < EndianReader < Endian , T2 > > for EndianReader < Endian , T1 > where Endian : Endianity , T1 : CloneStableDeref < Target = [u8] > + Debug , T2 : CloneStableDeref < Target = [u8] > + Debug , { fn eq (& self , rhs : & EndianReader < Endian , T2 >) -> bool { self . bytes () == rhs . bytes () } }
};
}
