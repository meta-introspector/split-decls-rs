// Generated macro for ElfFile32 (type)
macro_rules! Depcrate_read_elf_fileElfFile32 {
() => {
// Module: crate::read::elf::file
// Provides: {"ElfFile32"}
// Dependencies: {}
# [doc = " A 32-bit ELF object file."] # [doc = ""] # [doc = " This is a file that starts with [`elf::FileHeader32`], and corresponds"] # [doc = " to [`crate::FileKind::Elf32`]."] pub type ElfFile32 < 'data , Endian = Endianness , R = & 'data [u8] > = ElfFile < 'data , elf :: FileHeader32 < Endian > , R > ;
};
}
