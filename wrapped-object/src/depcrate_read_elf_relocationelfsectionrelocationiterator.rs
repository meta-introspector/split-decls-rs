// Generated macro for ElfSectionRelocationIterator (struct)
macro_rules! Depcrate_read_elf_relocationElfSectionRelocationIterator {
() => {
// Module: crate::read::elf::relocation
// Provides: {"ElfSectionRelocationIterator"}
// Dependencies: {}
# [doc = " An iterator for the relocations for an [`ElfSection`](super::ElfSection)."] pub struct ElfSectionRelocationIterator < 'data , 'file , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { # [doc = " The current pointer in the chain of relocation sections."] pub (super) section_index : SectionIndex , pub (super) file : & 'file ElfFile < 'data , Elf , R > , pub (super) relocations : Option < ElfRelocationIterator < 'data , Elf > > , }
};
}
