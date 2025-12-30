// Generated macro for SectionTable (struct)
macro_rules! Depcrate_read_elf_sectionSectionTable {
() => {
// Module: crate::read::elf::section
// Provides: {"SectionTable"}
// Dependencies: {}
# [doc = " The table of section headers in an ELF file."] # [doc = ""] # [doc = " Also includes the string table used for the section names."] # [doc = ""] # [doc = " Returned by [`FileHeader::sections`]."] # [derive (Debug , Clone , Copy)] pub struct SectionTable < 'data , Elf : FileHeader , R = & 'data [u8] > where R : ReadRef < 'data > , { sections : & 'data [Elf :: SectionHeader] , strings : StringTable < 'data , R > , }
};
}
