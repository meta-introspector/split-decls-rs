// Generated macro for ElfSectionIterator (struct)
macro_rules! Depcrate_read_elf_sectionElfSectionIterator {
() => {
// Module: crate::read::elf::section
// Provides: {"ElfSectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the sections in an [`ElfFile`]."] # [derive (Debug)] pub struct ElfSectionIterator < 'data , 'file , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { file : & 'file ElfFile < 'data , Elf , R > , iter : iter :: Enumerate < slice :: Iter < 'data , Elf :: SectionHeader > > , }
};
}
