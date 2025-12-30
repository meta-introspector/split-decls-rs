// Generated macro for impl_576 (impl)
macro_rules! Depcrate_read_elf_compressionimpl_576 {
() => {
// Module: crate::read::elf::compression
// Provides: {"impl_576"}
// Dependencies: {}
impl < Endian : endian :: Endian > CompressionHeader for elf :: CompressionHeader64 < Endian > { type Word = u64 ; type Endian = Endian ; # [inline] fn ch_type (& self , endian : Self :: Endian) -> u32 { self . ch_type . get (endian) } # [inline] fn ch_size (& self , endian : Self :: Endian) -> Self :: Word { self . ch_size . get (endian) } # [inline] fn ch_addralign (& self , endian : Self :: Endian) -> Self :: Word { self . ch_addralign . get (endian) } }
};
}
