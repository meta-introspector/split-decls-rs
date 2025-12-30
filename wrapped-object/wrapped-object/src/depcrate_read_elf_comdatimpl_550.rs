// Generated macro for impl_550 (impl)
macro_rules! Depcrate_read_elf_comdatimpl_550 {
() => {
// Module: crate::read::elf::comdat
// Provides: {"impl_550"}
// Dependencies: {}
impl < 'data , 'file , Elf , R > ElfComdat < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { fn parse (file : & 'file ElfFile < 'data , Elf , R > , section : & 'data Elf :: SectionHeader ,) -> Option < ElfComdat < 'data , 'file , Elf , R > > { let (flag , sections) = section . group (file . endian , file . data) . ok () ? ? ; if flag != elf :: GRP_COMDAT { return None ; } Some (ElfComdat { file , section , sections , }) } # [doc = " Get the ELF file containing this COMDAT section group."] pub fn elf_file (& self) -> & 'file ElfFile < 'data , Elf , R > { self . file } # [doc = " Get the raw ELF section header for the COMDAT section group."] pub fn elf_section_header (& self) -> & 'data Elf :: SectionHeader { self . section } }
};
}
