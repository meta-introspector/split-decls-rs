// Generated macro for impl_370 (impl)
macro_rules! Depcrate_read_endian_readerimpl_370 {
() => {
// Module: crate::read::endian_reader
// Provides: {"impl_370"}
// Dependencies: {}
impl < Endian , T > Hash for EndianReader < Endian , T > where Endian : Endianity , T : CloneStableDeref < Target = [u8] > + Debug , { fn hash < H : Hasher > (& self , state : & mut H) { self . bytes () . hash (state) ; } }
};
}
