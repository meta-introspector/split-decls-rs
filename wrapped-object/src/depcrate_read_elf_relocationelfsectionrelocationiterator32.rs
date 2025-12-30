// Generated macro for ElfSectionRelocationIterator32 (type)
macro_rules! Depcrate_read_elf_relocationElfSectionRelocationIterator32 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"ElfSectionRelocationIterator32"}
// Dependencies: {}
# [doc = " An iterator for the relocations for an [`ElfSection32`](super::ElfSection32)."] pub type ElfSectionRelocationIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSectionRelocationIterator < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
};
}
