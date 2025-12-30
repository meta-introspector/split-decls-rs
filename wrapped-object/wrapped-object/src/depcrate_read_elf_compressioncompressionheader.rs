// Generated macro for CompressionHeader (trait)
macro_rules! Depcrate_read_elf_compressionCompressionHeader {
() => {
// Module: crate::read::elf::compression
// Provides: {"CompressionHeader"}
// Dependencies: {}
# [doc = " A trait for generic access to [`elf::CompressionHeader32`] and [`elf::CompressionHeader64`]."] # [allow (missing_docs)] pub trait CompressionHeader : Debug + Pod { type Word : Into < u64 > ; type Endian : endian :: Endian ; fn ch_type (& self , endian : Self :: Endian) -> u32 ; fn ch_size (& self , endian : Self :: Endian) -> Self :: Word ; fn ch_addralign (& self , endian : Self :: Endian) -> Self :: Word ; }
};
}
