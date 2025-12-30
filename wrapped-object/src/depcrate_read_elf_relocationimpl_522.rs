// Generated macro for impl_522 (impl)
macro_rules! Depcrate_read_elf_relocationimpl_522 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"impl_522"}
// Dependencies: {}
impl < 'data , Elf : FileHeader > RelrIterator < 'data , Elf > { # [doc = " Create a new iterator given the `SHT_RELR` section data."] pub fn new (endian : Elf :: Endian , data : & 'data [Elf :: Relr]) -> Self { RelrIterator { offset : Elf :: Word :: default () , bits : Elf :: Word :: default () , count : 0 , iter : data . iter () , endian , } } }
};
}
