// Generated macro for ElfDynamicRelocationIterator32 (type)
macro_rules! Depcrate_read_elf_relocationElfDynamicRelocationIterator32 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"ElfDynamicRelocationIterator32"}
// Dependencies: {}
# [doc = " An iterator for the dynamic relocations in an [`ElfFile32`](super::ElfFile32)."] pub type ElfDynamicRelocationIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfDynamicRelocationIterator < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
};
}
