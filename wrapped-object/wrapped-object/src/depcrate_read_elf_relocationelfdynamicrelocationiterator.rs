// Generated macro for ElfDynamicRelocationIterator (struct)
macro_rules! Depcrate_read_elf_relocationElfDynamicRelocationIterator {
() => {
// Module: crate::read::elf::relocation
// Provides: {"ElfDynamicRelocationIterator"}
// Dependencies: {}
# [doc = " An iterator for the dynamic relocations in an [`ElfFile`]."] pub struct ElfDynamicRelocationIterator < 'data , 'file , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { # [doc = " The current relocation section index."] pub (super) section_index : SectionIndex , pub (super) file : & 'file ElfFile < 'data , Elf , R > , pub (super) relocations : Option < ElfRelocationIterator < 'data , Elf > > , }
};
}
