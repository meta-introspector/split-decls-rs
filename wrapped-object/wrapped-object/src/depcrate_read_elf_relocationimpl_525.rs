// Generated macro for impl_525 (impl)
macro_rules! Depcrate_read_elf_relocationimpl_525 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"impl_525"}
// Dependencies: {}
impl < Endian : endian :: Endian > Relr for elf :: Relr32 < Endian > { type Word = u32 ; type Endian = Endian ; const COUNT : u8 = 31 ; fn get (& self , endian : Self :: Endian) -> Self :: Word { self . 0 . get (endian) } fn next (offset : & mut Self :: Word , bits : & mut Self :: Word) -> Option < Self :: Word > { * offset += 4 ; * bits >>= 1 ; if * bits & 1 != 0 { Some (* offset) } else { None } } }
};
}
