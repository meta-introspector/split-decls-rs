// Generated macro for ElfDynamicRelocationIterator64 (type)
macro_rules! Depcrate_read_elf_relocationElfDynamicRelocationIterator64 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"ElfDynamicRelocationIterator64"}
// Dependencies: {}
# [doc = " An iterator for the dynamic relocations in an [`ElfFile64`](super::ElfFile64)."] pub type ElfDynamicRelocationIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfDynamicRelocationIterator < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
};
}
