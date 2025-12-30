// Generated macro for ElfFile (struct)
macro_rules! Depcrate_read_elf_fileElfFile {
() => {
// Module: crate::read::elf::file
// Provides: {"ElfFile"}
// Dependencies: {}
# [doc = " A partially parsed ELF file."] # [doc = ""] # [doc = " Most functionality is provided by the [`Object`] trait implementation."] # [derive (Debug)] pub struct ElfFile < 'data , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { pub (super) endian : Elf :: Endian , pub (super) data : R , pub (super) header : & 'data Elf , pub (super) segments : & 'data [Elf :: ProgramHeader] , pub (super) sections : SectionTable < 'data , Elf , R > , pub (super) relocations : RelocationSections , pub (super) symbols : SymbolTable < 'data , Elf , R > , pub (super) dynamic_symbols : SymbolTable < 'data , Elf , R > , }
};
}
