// Generated macro for ElfFile64 (type)
macro_rules! Depcrate_read_elf_fileElfFile64 {
() => {
// Module: crate::read::elf::file
// Provides: {"ElfFile64"}
// Dependencies: {}
# [doc = " A 64-bit ELF object file."] # [doc = ""] # [doc = " This is a file that starts with [`elf::FileHeader64`], and corresponds"] # [doc = " to [`crate::FileKind::Elf64`]."] pub type ElfFile64 < 'data , Endian = Endianness , R = & 'data [u8] > = ElfFile < 'data , elf :: FileHeader64 < Endian > , R > ;
};
}
