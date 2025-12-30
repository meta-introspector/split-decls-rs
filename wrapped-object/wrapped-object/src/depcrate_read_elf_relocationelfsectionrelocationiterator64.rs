// Generated macro for ElfSectionRelocationIterator64 (type)
macro_rules! Depcrate_read_elf_relocationElfSectionRelocationIterator64 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"ElfSectionRelocationIterator64"}
// Dependencies: {}
# [doc = " An iterator for the relocations for an [`ElfSection64`](super::ElfSection64)."] pub type ElfSectionRelocationIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSectionRelocationIterator < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
};
}
