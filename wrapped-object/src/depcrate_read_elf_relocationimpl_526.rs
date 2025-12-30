// Generated macro for impl_526 (impl)
macro_rules! Depcrate_read_elf_relocationimpl_526 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"impl_526"}
// Dependencies: {}
impl < Endian : endian :: Endian > Relr for elf :: Relr64 < Endian > { type Word = u64 ; type Endian = Endian ; const COUNT : u8 = 63 ; fn get (& self , endian : Self :: Endian) -> Self :: Word { self . 0 . get (endian) } fn next (offset : & mut Self :: Word , bits : & mut Self :: Word) -> Option < Self :: Word > { * offset += 8 ; * bits >>= 1 ; if * bits & 1 != 0 { Some (* offset) } else { None } } }
};
}
