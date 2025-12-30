// Generated macro for RelrIterator (struct)
macro_rules! Depcrate_read_elf_relocationRelrIterator {
() => {
// Module: crate::read::elf::relocation
// Provides: {"RelrIterator"}
// Dependencies: {}
# [doc = " An iterator over the relative relocations in an ELF `SHT_RELR` section."] # [doc = ""] # [doc = " Returned by [`SectionHeader::relr`](super::SectionHeader::relr)."] # [derive (Debug)] pub struct RelrIterator < 'data , Elf : FileHeader > { offset : Elf :: Word , bits : Elf :: Word , count : u8 , iter : slice :: Iter < 'data , Elf :: Relr > , endian : Elf :: Endian , }
};
}
