// Generated macro for impl_512 (impl)
macro_rules! Depcrate_read_elf_relocationimpl_512 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"impl_512"}
// Dependencies: {}
impl < 'data , 'file , Elf , R > Iterator for ElfSectionRelocationIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { type Item = (u64 , Relocation) ; fn next (& mut self) -> Option < Self :: Item > { let endian = self . file . endian ; loop { if let Some (ref mut relocations) = self . relocations { if let Some (reloc) = relocations . next () { let relocation = parse_relocation (self . file . header , endian , reloc , relocations . is_rel ()) ; return Some ((reloc . r_offset , relocation)) ; } self . relocations = None ; } self . section_index = self . file . relocations . get (self . section_index) ? ; let section = self . file . sections . section (self . section_index) . unwrap () ; match section . sh_type (endian) { elf :: SHT_REL => { if let Ok (relocations) = section . data_as_array (endian , self . file . data) { self . relocations = Some (ElfRelocationIterator :: Rel (relocations . iter () , endian)) ; } } elf :: SHT_RELA => { if let Ok (relocations) = section . data_as_array (endian , self . file . data) { self . relocations = Some (ElfRelocationIterator :: Rela (relocations . iter () , endian , self . file . header . is_mips64el (endian) ,)) ; } } elf :: SHT_CREL => { if let Ok (data) = section . data (endian , self . file . data) { if let Ok (relocations) = CrelIterator :: new (data) { self . relocations = Some (ElfRelocationIterator :: Crel (relocations)) ; } } } _ => { } } } } }
};
}
